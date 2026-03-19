"use client";

import { useParams, useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { CPUChart } from "@/components/dashboard/CPUChart";
import { RAMChart } from "@/components/dashboard/RAMChart";
import { ServiceStatusTabs } from "@/components/dashboard/ServiceStatusTabs";
import { api } from "@/lib/api";
import type { Node, ServiceStatusGrouped, NodeInfo } from "@/types";
import { Badge } from "@/components/ui/badge";
import { Header } from "@/components/dashboard/Header";
import { Button } from "@/components/ui/button";
import { toast } from "sonner";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  CheckCircle2,
  XCircle,
  Cpu,
  HardDrive,
  Info,
  Activity,
  ArrowLeft,
} from "lucide-react";

export default function NodeDetailsPage() {
  const params = useParams();
  const router = useRouter();
  const nodeId = Number(params.id);

  const [node, setNode] = useState<Node | null>(null);
  const [nodeInfo, setNodeInfo] = useState<NodeInfo | null>(null);
  const [services, setServices] = useState<ServiceStatusGrouped>({});
  const [loadingServices, setLoadingServices] = useState(false);
  const [loadingInfo, setLoadingInfo] = useState(false);

  useEffect(() => {
    fetchNode();
  }, [nodeId]);

  const fetchNode = async () => {
    try {
      const nodes = await api.getNodes();
      const selected = nodes.find((n: Node) => n.id === nodeId);
      setNode(selected || null);
      if (selected) {
        fetchNodeInfo();
        fetchServices();
      }
    } catch (error) {
      console.error("Error fetching node:", error);
    }
  };

  const fetchNodeInfo = async () => {
    try {
      setLoadingInfo(true);
      const info = await api.getNodeInfo(nodeId);
      setNodeInfo(info);
    } catch (error) {
      console.error("Error fetching node info:", error);
    } finally {
      setLoadingInfo(false);
    }
  };

  const fetchServices = async () => {
    try {
      setLoadingServices(true);

      const response = await api.getServiceStatus(nodeId);
      setServices(response || {});
    } catch (error) {
      console.error("Error fetching services:", error);
    } finally {
      setLoadingServices(false);
    }
  };

  if (!node) {
    return (
      <div className="p-10 text-center text-muted-foreground">
        Loading node...
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-linear-to-br from-background via-background to-muted/20">
      <Header
        onRefresh={() => {
          fetchNode();
          toast.success("Node data refreshed");
        }}
        isLoading={loadingInfo || loadingServices}
      />

      <main className="px-7 py-6 space-y-6">
        <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
          <div className="flex items-center gap-4">
            <Button
              variant="outline"
              size="icon"
              onClick={() => router.push("/")}
              className="rounded-full h-10 w-10 shrink-0"
              aria-label="Back to dashboard"
            >
              <ArrowLeft className="h-5 w-5" />
            </Button>
            <h1 className="text-3xl font-bold tracking-tight">{node.name}</h1>
          </div>
          <div className="flex items-center gap-2">
            <Badge variant="outline" className="text-sm px-3 py-1">
              Node ID: {node.id}
            </Badge>
            {nodeInfo && (
              <Badge variant="secondary" className="text-sm px-3 py-1">
                {nodeInfo.os_version}
              </Badge>
            )}
          </div>
        </div>

        <div className="grid grid-cols-1 xl:grid-cols-3 gap-6">
          <div className="xl:col-span-2 space-y-6">
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
              <CPUChart nodeId={node.id} nodeName={node.name} />
              <RAMChart nodeId={node.id} nodeName={node.name} />
            </div>

            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Activity className="w-5 h-5" />
                  Services Status
                </CardTitle>
              </CardHeader>
              <CardContent>
                {loadingServices ? (
                  <div className="text-center py-4 text-muted-foreground italic">
                    Loading services...
                  </div>
                ) : (
                  <ServiceStatusTabs services={services} />
                )}
              </CardContent>
            </Card>
          </div>

          <div className="space-y-6">
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Info className="w-5 h-5" />
                  System Information
                </CardTitle>
                <CardDescription>
                  Detailed hardware and OS specifications
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                {loadingInfo ? (
                  <div className="text-center py-8 text-muted-foreground italic">
                    Loading system info...
                  </div>
                ) : nodeInfo ? (
                  <>
                    <div className="grid grid-cols-2 gap-4">
                      <div className="p-3 rounded-lg border bg-accent/30 space-y-1">
                        <span className="text-xs text-muted-foreground uppercase font-semibold">
                          OS Name
                        </span>
                        <p className="font-medium truncate">
                          {nodeInfo.system_name}
                        </p>
                      </div>
                      <div className="p-3 rounded-lg border bg-accent/30 space-y-1">
                        <span className="text-xs text-muted-foreground uppercase font-semibold">
                          Uptime
                        </span>
                        <p className="font-medium">
                          {Math.floor(nodeInfo.uptime / 3600)}h{" "}
                          {Math.floor((nodeInfo.uptime % 3600) / 60)}m
                        </p>
                      </div>
                    </div>

                    <div className="space-y-3">
                      <div className="flex items-start gap-3 p-3 rounded-lg border">
                        <Cpu className="w-5 h-5 mt-0.5 text-blue-500" />
                        <div className="space-y-1">
                          <span className="text-sm font-semibold">
                            CPU Details
                          </span>
                          <p className="text-sm text-muted-foreground">
                            {nodeInfo.cpu_vendor}
                          </p>
                          <Badge variant="secondary" className="text-xs">
                            {nodeInfo.cpu_threads} Threads
                          </Badge>
                        </div>
                      </div>

                      <div className="flex items-start gap-3 p-3 rounded-lg border">
                        <HardDrive className="w-5 h-5 mt-0.5 text-purple-500" />
                        <div className="space-y-1">
                          <span className="text-sm font-semibold">
                            Kernel Version
                          </span>
                          <p className="text-sm text-muted-foreground font-mono truncate">
                            {nodeInfo.kernel_version}
                          </p>
                        </div>
                      </div>

                      <div className="flex items-start gap-3 p-3 rounded-lg border">
                        <Info className="w-5 h-5 mt-0.5 text-emerald-500" />
                        <div className="space-y-1">
                          <span className="text-sm font-semibold">
                            OS Version
                          </span>
                          <p className="text-sm text-muted-foreground">
                            {nodeInfo.os_version}
                          </p>
                        </div>
                      </div>
                    </div>
                  </>
                ) : (
                  <div className="text-center py-8 text-muted-foreground italic border rounded-lg border-dashed">
                    System information not available
                  </div>
                )}
              </CardContent>
            </Card>
          </div>
        </div>
      </main>
    </div>
  );
}
