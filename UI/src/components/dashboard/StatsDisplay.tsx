"use client";

import { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Card, CardContent } from "@/components/ui/card";
import { Activity, Cpu, HardDrive, Server, Plus, Copy, Check, X } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { api } from "@/lib/api";

interface StatsDisplayProps {
  totalNodes: number;
  averageCPU: number;
  totalRAM: { used: number; total: number };
}

export function StatsDisplay({ totalNodes, averageCPU, totalRAM }: StatsDisplayProps) {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [nodeName, setNodeName] = useState("");
  const [token, setToken] = useState("");
  const [isCopied, setIsCopied] = useState(false);
  const [isLoading, setIsLoading] = useState(false);

  const handleCreateNode = async () => {
    if (!nodeName.trim()) return;
    try {
      setIsLoading(true);
      const res = await api.createNode(nodeName);
      if (res && res.token) {
        setToken(res.token);
      }
    } catch (error) {
      console.error("Failed to create node:", error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(token);
    setIsCopied(true);
    setTimeout(() => setIsCopied(false), 2000);
  };

  const resetModal = () => {
    setIsModalOpen(false);
    setNodeName("");
    setToken("");
    setIsCopied(false);
  };

  const stats = [
    {
      label: "Total Nodes",
      value: totalNodes.toString(),
      icon: Server,
      color: "text-blue-500",
      bgColor: "bg-blue-50 dark:bg-blue-950",
    },
    {
      label: "Average CPU",
      value: `${averageCPU.toFixed(1)}%`,
      icon: Cpu,
      color: "text-purple-500",
      bgColor: "bg-purple-50 dark:bg-purple-950",
    },
    {
      label: "Total RAM",
      value: `${totalRAM.used.toFixed(1)}/${totalRAM.total.toFixed(1)} GiB`,
      icon: HardDrive,
      color: "text-green-500",
      bgColor: "bg-green-50 dark:bg-green-950",
    },
    {
      label: "System Status",
      value: "Operational",
      icon: Activity,
      color: "text-emerald-500",
      bgColor: "bg-emerald-50 dark:bg-emerald-950",
    },
  ];

  return (
    <>
      <div className="flex justify-end mb-4">
        <Button onClick={() => setIsModalOpen(true)} className="flex items-center gap-2">
          <Plus className="h-4 w-4" />
          Create Node
        </Button>
      </div>

      <AnimatePresence>
        {isModalOpen && (
          <>
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              onClick={resetModal}
              className="fixed inset-0 bg-black/70 z-[100]"
            />
            <div className="fixed inset-0 flex items-center justify-center z-[101] p-4 pointer-events-none overflow-y-auto">
              <motion.div
                initial={{ opacity: 0, scale: 0.95, y: 10 }}
                animate={{ opacity: 1, scale: 1, y: 0 }}
                exit={{ opacity: 0, scale: 0.95, y: 10 }}
                className="w-full max-w-md pointer-events-auto"
              >
                <Card className="w-full p-6 border-primary/20 bg-card shadow-2xl overflow-hidden">
                  <div className="flex justify-between items-center mb-6">
                    <h2 className="text-xl font-semibold">Create New Node</h2>
                    <Button variant="ghost" size="icon" onClick={resetModal}>
                      <X className="h-4 w-4" />
                    </Button>
                  </div>

                  {!token ? (
                    <div className="space-y-4">
                      <div>
                        <label className="text-sm font-medium mb-1 block">Node Name</label>
                        <Input
                          placeholder="Enter node name"
                          value={nodeName}
                          onChange={(e) => setNodeName(e.target.value)}
                        />
                      </div>
                      <Button
                        className="w-full"
                        onClick={handleCreateNode}
                        disabled={!nodeName.trim() || isLoading}
                      >
                        {isLoading ? "Creating..." : "Create"}
                      </Button>
                    </div>
                  ) : (
                    <div className="space-y-4 animate-in fade-in">
                      <div className="p-4 bg-muted rounded-md break-all relative">
                        <p className="text-sm font-mono pr-10">{token}</p>
                        <Button
                          variant="ghost"
                          size="icon"
                          className="absolute top-2 right-2 h-8 w-8"
                          onClick={handleCopy}
                        >
                          {isCopied ? <Check className="h-4 w-4 text-green-500" /> : <Copy className="h-4 w-4" />}
                        </Button>
                      </div>
                      <Button className="w-full" onClick={resetModal}>
                        Close
                      </Button>
                    </div>
                  )}
                </Card>
              </motion.div>
            </div>
          </>
        )}
      </AnimatePresence>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
        {stats.map((stat, index) => (
          <motion.div
            key={stat.label}
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: index * 0.05 }}
          >
            <Card className="hover:shadow-md transition-shadow">
              <CardContent className="p-6">
                <div className="flex items-center justify-between">
                  <div>
                    <p className="text-sm font-medium text-muted-foreground">
                      {stat.label}
                    </p>
                    <p className="text-2xl font-bold mt-1">{stat.value}</p>
                  </div>
                  <div className={`p-3 rounded-full ${stat.bgColor}`}>
                    <stat.icon className={`h-5 w-5 ${stat.color}`} />
                  </div>
                </div>
              </CardContent>
            </Card>
          </motion.div>
        ))}
      </div>
    </>
  );
}