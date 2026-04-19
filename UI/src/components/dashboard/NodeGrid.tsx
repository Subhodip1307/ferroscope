"use client";

import { useState, useEffect, useRef } from "react";
import autoAnimate from "@formkit/auto-animate";
import { NodeCard } from "./NodeCard";
import type { Node } from "@/types";
import { motion, AnimatePresence } from "framer-motion";
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { AlertTriangle, X } from "lucide-react";
import { api } from "@/lib/api";
import { toast } from "sonner";

interface NodeGridProps {
  nodes: Node[];
  onDelete?: () => void;
}

export function NodeGrid({ nodes, onDelete }: NodeGridProps) {
  const [nodeToDelete, setNodeToDelete] = useState<Node | null>(null);
  const [isDeleting, setIsDeleting] = useState(false);
  const gridRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (gridRef.current) {
      autoAnimate(gridRef.current);
    }
  }, []);

  const handleDelete = async () => {
    if (!nodeToDelete) return;
    try {
      setIsDeleting(true);
      const success = await api.removeNode(nodeToDelete.id);
      if (success) {
        toast.success(`Node "${nodeToDelete.name}" deleted successfully`);
        onDelete?.();
        setNodeToDelete(null);
      } else {
        toast.error("Failed to delete node");
      }
    } catch (error) {
      console.error("Delete error:", error);
      toast.error("An error occurred while deleting the node");
    } finally {
      setIsDeleting(false);
    }
  };

  return (
    <>
      <div 
        ref={gridRef}
        className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4"
      >
        {nodes.map((node, index) => (
          <NodeCard key={node.id} node={node} index={index} onDelete={setNodeToDelete} />
        ))}
      </div>

      <AnimatePresence>
        {nodeToDelete && (
          <>
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              onClick={() => setNodeToDelete(null)}
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
                    <div className="flex items-center gap-2">
                      <div className="p-2 rounded-full bg-destructive/10 text-destructive">
                        <AlertTriangle className="h-5 w-5" />
                      </div>
                      <h2 className="text-xl font-semibold">Delete Node?</h2>
                    </div>
                    <Button variant="ghost" size="icon" onClick={() => setNodeToDelete(null)}>
                      <X className="h-4 w-4" />
                    </Button>
                  </div>

                  <div className="space-y-6">
                    <p className="text-sm text-muted-foreground">
                      Are you sure you want to delete <span className="font-semibold text-foreground">"{nodeToDelete.name}"</span>? 
                      This will permanently remove all associated data and monitoring history.
                    </p>

                    <div className="flex gap-3 pt-2">
                      <Button
                        variant="outline"
                        className="flex-1"
                        onClick={() => setNodeToDelete(null)}
                        disabled={isDeleting}
                      >
                        Cancel
                      </Button>
                      <Button
                        variant="destructive"
                        className="flex-1"
                        onClick={handleDelete}
                        disabled={isDeleting}
                      >
                        {isDeleting ? "Deleting..." : "Delete Node"}
                      </Button>
                    </div>
                  </div>
                </Card>
              </motion.div>
            </div>
          </>
        )}
      </AnimatePresence>
    </>
  );
}