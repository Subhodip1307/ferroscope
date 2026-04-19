"use client";

import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import { motion, AnimatePresence } from "framer-motion";
import { Header } from "@/components/dashboard/Header";
import { NodeGrid } from "@/components/dashboard/NodeGrid";
import { StatsDisplay } from "@/components/dashboard/StatsDisplay";
import { api } from "@/lib/api";
import { parseRAM } from "@/lib/utils";
import { toast } from "sonner";
import type { Node } from "@/types";

export default function DashboardPage() {
  const router = useRouter();
  const [nodes, setNodes] = useState<Node[]>([]);
  const [selectedNode, setSelectedNode] = useState<Node | null>(null);
  const [loading, setLoading] = useState(true);
  const [stats, setStats] = useState({
    totalNodes: 0,
    averageCPU: 0,
    totalRAM: { used: 0, total: 0 },
  });

  useEffect(() => {
    const token = localStorage.getItem("ferro_token");
    if (!token) {
      router.push("/login");
      return;
    }
    fetchNodes();
  }, []);

  useEffect(() => {
    if (nodes.length > 0 && !selectedNode) {
      setSelectedNode(nodes[0]);
    }
    calculateStats();
  }, [nodes]);

  const fetchNodes = async () => {
    try {
      setLoading(true);
      const nodeList = await api.getNodes();
      setNodes(nodeList);
    } catch (error) {
      console.error("Error fetching nodes:", error);
      toast.error("Failed to fetch nodes");
    } finally {
      setLoading(false);
    }
  };

  const calculateStats = async () => {
    if (nodes.length === 0) return;

    try {
      const cpuPromises = nodes.map(node => api.getLatestCPU(node.id));
      const ramPromises = nodes.map(node => api.getLatestRAM(node.id));

      const cpuResults = await Promise.all(cpuPromises);
      const ramResultsRaw = await Promise.all(ramPromises);
      const ramResults = ramResultsRaw.filter(Boolean);

      const avgCPU = cpuResults.reduce((acc, curr) => acc + curr.cpu, 0) / cpuResults.length;

      const totalRAMUsed = ramResults.reduce((acc, curr) => {
        if (!curr) return acc;
        return acc + (parseRAM(curr.total) - parseRAM(curr.free));
      }, 0);

      const totalRAMTotal = ramResults.reduce((acc, curr) => {
        if (!curr) return acc;
        return acc + parseRAM(curr.total);
      }, 0);

      setStats({
        totalNodes: nodes.length,
        averageCPU: avgCPU,
        totalRAM: { used: totalRAMUsed, total: totalRAMTotal },
      });
    } catch (error) {
      console.error("Error calculating stats:", error);
    }
  };

  const handleRefresh = () => {
    fetchNodes();
    toast.success("Dashboard data refreshed", { closeButton: true });
  };

  return (
    <div className="min-h-screen bg-linear-to-br from-background via-background to-muted/20">
      <Header onRefresh={handleRefresh} isLoading={loading} />

      <main className="container mx-auto px-4 py-6">
        <AnimatePresence mode="wait">
          {loading ? (
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className="flex items-center justify-center h-[60vh]"
            >
              <div className="text-center space-y-4">
                <div className="w-16 h-16 border-4 border-primary border-t-transparent rounded-full animate-spin mx-auto" />
                <p className="text-muted-foreground">Loading dashboard...</p>
              </div>
            </motion.div>
          ) : (
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className="space-y-6"
            >
              <StatsDisplay
                totalNodes={stats.totalNodes}
                averageCPU={stats.averageCPU}
                totalRAM={stats.totalRAM}
                onRefresh={handleRefresh}
              />

              <NodeGrid nodes={nodes} onDelete={fetchNodes} />
            </motion.div>
          )}
        </AnimatePresence>
      </main>
    </div>
  );
}