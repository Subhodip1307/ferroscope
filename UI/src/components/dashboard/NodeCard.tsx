"use client";

import { useState, useEffect } from "react";
import { motion } from "framer-motion";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Progress } from "@/components/ui/progress";
import { Cpu, HardDrive, TrendingUp, TrendingDown, Trash2, RefreshCw } from "lucide-react";
import { api } from "@/lib/api";
import { parseRAM } from "@/lib/utils";
import type { Node, CPUData, RAMData } from "@/types";
import { useRouter } from "next/navigation";
import { toast } from "sonner";
import { Button } from "../ui/button";


interface NodeCardProps {
  node: Node;
  index: number;
  onDelete?: (node: Node) => void;
}

export function NodeCard({ node, index, onDelete }: NodeCardProps) {
  const [cpuData, setCpuData] = useState<CPUData | null>(null);
  const [ramData, setRamData] = useState<RAMData | null>(null);
  const [loading, setLoading] = useState(true);
  const [isOffline, setIsOffline] = useState(false);
  const [isRefreshing, setIsRefreshing] = useState(false);
  const [streamKey, setStreamKey] = useState(0);
  const router = useRouter();

  const fetchNodeData = async () => {
    try {
      setIsRefreshing(true);
      const [cpu, ram] = await Promise.all([
        api.getLatestCPU(node.id),
        api.getLatestRAM(node.id),
      ]);
      setCpuData(cpu);
      setRamData(ram);
      setIsOffline(false);
    } catch (error: any) {
      console.error("Error fetching node data:", error);
      if (error.message?.includes("503")) {
        setIsOffline(true);
      }
    } finally {
      setIsRefreshing(false);
      setLoading(false);
    }
  };

  const handleManualRefresh = async () => {
    await fetchNodeData();
    setStreamKey(prev => prev + 1);
  };

  useEffect(() => {
    fetchNodeData();

    // Connect to SSE for real-time CPU updates
    const cpuStreamUrl = api.getCPUStreamUrl(node.id);
    const cpuEventSource = new EventSource(cpuStreamUrl);

    cpuEventSource.onmessage = (event) => {
      try {
        setIsOffline(false);
        const newData = JSON.parse(event.data);
        setCpuData({
          cpu: newData.value,
          timestamp: newData.date_time
        });
      } catch (err) {
        console.error("Error parsing CPU stream data:", err);
      }
    };

    cpuEventSource.onerror = (err) => {
      // EventSource automatically retries.
      setIsOffline(true);
    };

    // Connect to SSE for real-time RAM updates
    const ramStreamUrl = api.getRAMStreamUrl(node.id);
    const ramEventSource = new EventSource(ramStreamUrl);

    ramEventSource.onmessage = (event) => {
      try {
        setIsOffline(false);
        const newData = JSON.parse(event.data);
        setRamData({
          free: newData.free,
          total: newData.total,
          timestamp: newData.timestamp
        });
      } catch (err) {
        console.error("Error parsing RAM stream data:", err);
      }
    };

    ramEventSource.onerror = (err) => {
      setIsOffline(true);
    };

    return () => {
      cpuEventSource.close();
      ramEventSource.close();
    };
  }, [node.id, streamKey]);

  const ramUsagePercent = ramData
    ? ((parseRAM(ramData.total) - parseRAM(ramData.free)) /
        parseRAM(ramData.total)) *
      100
    : 0;

  const cpuStatus = cpuData?.cpu ?? 0;

  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ delay: index * 0.1 }}
      whileHover={{ scale: 1.02 }}
      className="h-full"
    >
      <Card 
      onClick={() => router.push(`/nodes/${node.id}`)}
      className="h-full hover:shadow-lg transition-shadow duration-300 overflow-hidden">
        <CardHeader className="pb-2">
          <div className="flex items-center justify-between">
            <div className="flex flex-col">
              <CardTitle className="text-lg font-semibold">{node.name}</CardTitle>
              <div className="flex items-center gap-2 mt-1">
                <Badge variant={isOffline ? "destructive" : (cpuStatus > 80 ? "destructive" : "secondary")}>
                  {isOffline ? "Offline" : (cpuStatus > 80 ? "High Load" : "Normal")}
                </Badge>
              </div>
            </div>
            <div className="flex items-center gap-1">
              <Button
                variant="ghost"
                size="icon"
                className="text-muted-foreground hover:text-primary hover:bg-primary/10 transition-colors"
                onClick={(e) => {
                  e.stopPropagation();
                  handleManualRefresh();
                }}
                disabled={isRefreshing}
              >
                <RefreshCw className={`h-4 w-4 ${isRefreshing ? "animate-spin" : ""}`} />
              </Button>
              <Button
                variant="ghost"
                size="icon"
                className="text-muted-foreground hover:text-destructive hover:bg-destructive/10 transition-colors"
                onClick={(e) => {
                  e.stopPropagation();
                  onDelete?.(node);
                }}
              >
                <Trash2 className="h-4 w-4" />
              </Button>
            </div>
          </div>
        </CardHeader>

        <CardContent className={`space-y-4 ${isOffline ? "opacity-50 grayscale" : ""}`}>
          <motion.div
            className="space-y-2"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ delay: 0.2 }}
          >
            <div className="flex items-center justify-between text-sm">
              <div className="flex items-center gap-2">
                <Cpu className="h-4 w-4 text-blue-500" />
                <span className="font-medium">CPU Usage</span>
              </div>
              <span className="font-bold">
                {(cpuData?.cpu ?? 0).toFixed(1)}%
              </span>
            </div>
            <Progress value={cpuData?.cpu ?? 0} />
            
          </motion.div>

          <motion.div
            className="space-y-2"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ delay: 0.3 }}
          >
            <div className="flex items-center justify-between text-sm">
              <div className="flex items-center gap-2">
                <HardDrive className="h-4 w-4 text-green-500" />
                <span className="font-medium">RAM Usage</span>
              </div>
              <span className="text-xs text-muted-foreground font-medium">
                {ramData ? `${(parseRAM(ramData.total) - parseRAM(ramData.free)).toFixed(2)} GiB / ${ramData.total}` : "Loading..."}
              </span>
            </div>
            <Progress value={ramUsagePercent} className="h-2" />
          </motion.div>

          {cpuData && (
            <div className="flex items-center justify-center pt-2">
              {cpuStatus > 50 ? (
                <TrendingUp className="h-5 w-5 text-red-500 animate-pulse" />
              ) : (
                <TrendingDown className="h-5 w-5 text-green-500" />
              )}
            </div>
          )}
        </CardContent>
      </Card>
    </motion.div>
  );
}
