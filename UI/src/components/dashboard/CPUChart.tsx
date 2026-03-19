"use client";

import { useEffect, useState } from "react";
import { motion } from "framer-motion";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { AreaChart, Area, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from "recharts";
import { api } from "@/lib/api";
import { formatTimestamp } from "@/lib/utils";
import type { CPUData } from "@/types";

interface CPUChartProps {
  nodeId: number;
  nodeName: string;
}

export function CPUChart({ nodeId, nodeName }: CPUChartProps) {
  const [data, setData] = useState<CPUData[]>([]);
  const [loading, setLoading] = useState(true);
  const [isOffline, setIsOffline] = useState(false);

  useEffect(() => {
    fetchHistory();

    // Connect to SSE for real-time updates
    const streamUrl = api.getCPUStreamUrl(nodeId);
    const eventSource = new EventSource(streamUrl);

    eventSource.onmessage = (event) => {
      try {
        setIsOffline(false);
        const newData = JSON.parse(event.data);
        const formattedPoint: CPUData = {
          cpu: newData.value,
          timestamp: newData.date_time
        };

        setData(prevData => {
          // Keep only last 20 points for smooth performance
          const updated = [formattedPoint, ...prevData];
          return updated.slice(0, 20);
        });
      } catch (err) {
        console.error("Error parsing CPU stream data:", err);
      }
    };

    eventSource.onerror = (err) => {
      setIsOffline(true);
    };

    return () => {
      eventSource.close();
    };
  }, [nodeId]);

  const fetchHistory = async () => {
    try {
      const cpuHistory = await api.getCPUHistory(nodeId);
      setData(cpuHistory.slice(0, 20)); // Initial state with last 20 points
      setIsOffline(false);
    } catch (error: any) {
      console.error("Error fetching CPU history:", error);
      if (error.message?.includes("503")) {
        setIsOffline(true);
      }
    } finally {
      setLoading(false);
    }
  };

  const chartData = [...data].map(item => ({
    timestamp: item.timestamp,
    time: formatTimestamp(item.timestamp),
    cpu: item.cpu,
  })).reverse(); // Keep reverse here as Recharts typically expects chronological order (left to right)

  return (
    <motion.div
      initial={{ opacity: 0, x: -20 }}
      animate={{ opacity: 1, x: 0 }}
      transition={{ duration: 0.5 }}
    >
      <Card>
        <CardHeader>
          <CardTitle className="text-lg">CPU History - {nodeName}</CardTitle>
        </CardHeader>
        <CardContent>
          {loading ? (
            <div className="h-[300px] flex items-center justify-center">
              <div className="animate-pulse text-muted-foreground">Loading chart...</div>
            </div>
          ) : isOffline ? (
            <div className="h-[300px] flex flex-col items-center justify-center space-y-2">
              <div className="text-destructive font-semibold">Node Offline</div>
              <div className="text-muted-foreground text-sm text-center px-4">
                Wait for the node to come back online for real-time data.
              </div>
            </div>
          ) : (
            <ResponsiveContainer width="100%" height={300}>
              <AreaChart data={chartData}>
                <defs>
                  <linearGradient id="cpuGradient" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="#3b82f6" stopOpacity={0.8} />
                    <stop offset="95%" stopColor="#3b82f6" stopOpacity={0} />
                  </linearGradient>
                </defs>
                <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
                <XAxis
                  dataKey="timestamp"
                  className="text-xs"
                  stroke="currentColor"
                  tickFormatter={formatTimestamp}
                  minTickGap={30}
                />
                <YAxis
                  domain={[0, 100]}
                  className="text-xs"
                  stroke="currentColor"
                />
                <Tooltip
                  contentStyle={{
                    backgroundColor: "hsl(var(--background))",
                    border: "1px solid hsl(var(--border))",
                    borderRadius: "6px",
                  }}
                  labelFormatter={(label) => formatTimestamp(String(label))}
                />
                <Area
                  type="monotone"
                  dataKey="cpu"
                  stroke="#3b82f6"
                  fillOpacity={1}
                  fill="url(#cpuGradient)"
                  strokeWidth={2}
                  isAnimationActive={false}
                />
              </AreaChart>
            </ResponsiveContainer>
          )}
        </CardContent>
      </Card>
    </motion.div>
  );
}