"use client";

import { useState, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { X, Lock, CheckCircle2, AlertCircle, ShieldCheck } from "lucide-react";
import { toast } from "sonner";
import { api } from "@/lib/api";
import type {
  UserAccessControlItem,
  UserPermissionsResponse,
  MetricType,
  NodeWithServices,
} from "@/types";

interface PermissionsModalProps {
  isOpen: boolean;
  user: UserAccessControlItem | null;
  onClose: () => void;
  onSuccess: () => void;
}

interface PermissionForm {
  [nodeId: number]: {
    full_permission: boolean;
    metrics: {
      RAM: boolean;
      CPU: boolean;
      DISK: boolean;
    };
    services: number[];
  };
}

// Permission categories shown in the compact admin view.
// Adjust this list if your app tracks other permission types.
const ADMIN_PERMISSION_LABELS = [
  "All Nodes",
  "RAM Metrics",
  "CPU Metrics",
  "DISK Metrics",
  "All Services",
];

export function PermissionsModal({
  isOpen,
  user,
  onClose,
  onSuccess,
}: PermissionsModalProps) {
  const [form, setForm] = useState<PermissionForm>({});
  const [isLoading, setIsLoading] = useState(false);
  const [isFetching, setIsFetching] = useState(false);
  const [nodesWithServices, setNodesWithServices] = useState<
    NodeWithServices[]
  >([]);

  // NOTE: assumes `user.is_admin` exists on UserAccessControlItem.
  // Swap this for the correct field (e.g. user.role === "admin") if different.
  const isAdmin = Boolean(user?.is_admin);

  useEffect(() => {
    if (isOpen && user && !isAdmin) {
      fetchNodesAndPermissions();
    }
  }, [isOpen, user, isAdmin]);

  const fetchNodesAndPermissions = async () => {
    if (!user) return;
    setIsFetching(true);
    try {
      // Fetch all nodes
      const allNodes = await api.getNodes();
      const nodeIds = allNodes.map((n) => n.id);

      // Fetch nodes with services
      const nodesWithServs = await api.getNodesWithServices(nodeIds);
      setNodesWithServices(nodesWithServs);

      // Try to get permissions from user object first, if not fetch from API
      let userPerms = user.permissions;
      if (!userPerms) {
        const fetchedPerms = await api.getUserPermissions(user.id);
        userPerms = fetchedPerms ?? undefined; // ← convert null to undefined
      }

      // Initialize form with permissions
      if (userPerms) {
        initializeFormFromPermissions(userPerms, nodesWithServs);
      } else {
        initializeEmptyForm(nodesWithServs);
      }
    } catch (error) {
      console.error("Error fetching nodes and permissions:", error);
      toast.error("Failed to load nodes");
      // Still initialize empty form so user can set new permissions
      if (nodesWithServices.length > 0) {
        initializeEmptyForm(nodesWithServices);
      }
    } finally {
      setIsFetching(false);
    }
  };

  const initializeEmptyForm = (nodes: NodeWithServices[]) => {
    const newForm: PermissionForm = {};
    nodes.forEach((node) => {
      newForm[node.node_id] = {
        full_permission: false,
        metrics: { RAM: false, CPU: false, DISK: false },
        services: [],
      };
    });
    setForm(newForm);
  };

  const initializeFormFromPermissions = (
    perms: UserPermissionsResponse,
    nodes: NodeWithServices[],
  ) => {
    const newForm: PermissionForm = {};

    nodes.forEach((node) => {
      const nodePerm = perms.nodes_permissions.find(
        (p) => p.node_id === node.node_id,
      );

      newForm[node.node_id] = {
        full_permission: nodePerm?.is_full_access ?? false,
        metrics: {
          RAM: nodePerm?.metrix.includes("RAM") ?? false,
          CPU: nodePerm?.metrix.includes("CPU") ?? false,
          DISK: nodePerm?.metrix.includes("DISK") ?? false,
        },
        services: nodePerm?.services ?? [],
      };
    });

    setForm(newForm);
  };

  const handleMetricChange = (
    nodeId: number,
    metric: MetricType,
    checked: boolean,
  ) => {
    setForm((prev) => ({
      ...prev,
      [nodeId]: {
        ...prev[nodeId],
        metrics: {
          ...prev[nodeId].metrics,
          [metric]: checked,
        },
      },
    }));
  };

  const handleServiceToggle = (nodeId: number, serviceId: number) => {
    setForm((prev) => ({
      ...prev,
      [nodeId]: {
        ...prev[nodeId],
        services: prev[nodeId].services.includes(serviceId)
          ? prev[nodeId].services.filter((id) => id !== serviceId)
          : [...prev[nodeId].services, serviceId],
      },
    }));
  };

  const handleFullPermissionToggle = (nodeId: number, checked: boolean) => {
    setForm((prev) => ({
      ...prev,
      [nodeId]: {
        ...prev[nodeId],
        full_permission: checked,
        metrics: checked
          ? { RAM: false, CPU: false, DISK: false }
          : prev[nodeId].metrics,
        services: checked ? [] : prev[nodeId].services,
      },
    }));
  };

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!user) return;

    setIsLoading(true);
    try {
      const nodes_permissions = Object.entries(form).map(
        ([nodeId, nodePerms]) => ({
          node_id: parseInt(nodeId, 10),
          metrix: nodePerms.full_permission
            ? null
            : Object.entries(nodePerms.metrics)
                .filter(([, checked]) => checked)
                .map(([metric]) => metric as MetricType),
          services: nodePerms.full_permission
            ? null
            : nodePerms.services.length > 0
              ? nodePerms.services
              : null,
          full_permission: nodePerms.full_permission || null,
        }),
      );

      const success = await api.assignPermissions({
        user_id: user.id,
        nodes_permissions,
      });

      if (success) {
        toast.success(`Permissions assigned to @${user.username}`);
        onSuccess();
        onClose();
      } else {
        toast.error("Failed to assign permissions");
      }
    } catch (error: unknown) {
      const err = error as Error;
      console.error("Error assigning permissions:", error);
      toast.error(
        err.message || "An error occurred while assigning permissions",
      );
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <AnimatePresence>
      {isOpen && user && (
        <>
          {/* Backdrop */}
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            onClick={onClose}
            className="fixed inset-0 bg-black/70 z-[100]"
          />

          {/* Modal */}
          <div className="fixed inset-0 flex items-center justify-center z-[101] p-4 pointer-events-none">
            {isAdmin ? (
              // ---- Compact admin view ----
              <motion.div
                initial={{ opacity: 0, scale: 0.95, y: 10 }}
                animate={{ opacity: 1, scale: 1, y: 0 }}
                exit={{ opacity: 0, scale: 0.95, y: 10 }}
                className="w-full max-w-sm pointer-events-auto"
              >
                <Card className="border-primary/20 bg-card shadow-2xl overflow-hidden">
                  <CardHeader className="relative pb-2">
                    <Button
                      variant="ghost"
                      size="icon"
                      onClick={onClose}
                      className="absolute right-4 top-4 rounded-full h-8 w-8 text-muted-foreground hover:text-foreground"
                    >
                      <X className="h-4 w-4" />
                    </Button>
                    <CardTitle className="flex items-center gap-2.5 text-xl">
                      <ShieldCheck className="w-5 h-5 text-primary" />
                      Permissions
                    </CardTitle>
                    <CardDescription>
                      <span className="font-semibold text-foreground">
                        @{user.username}
                      </span>{" "}
                      is an admin
                    </CardDescription>
                  </CardHeader>

                  <CardContent className="space-y-3 pt-2">
                    <div className="p-3 rounded-lg bg-primary/10 border border-primary/20 text-xs text-primary flex items-center gap-2">
                      <CheckCircle2 className="w-4 h-4 shrink-0" />
                      Admins have full access to all permissions by default
                    </div>

                    <div className="flex flex-wrap gap-2">
                      {ADMIN_PERMISSION_LABELS.map((label) => (
                        <span
                          key={label}
                          className="text-xs font-medium px-2.5 py-1 rounded-full border border-border/50 bg-muted/30 text-foreground/80"
                        >
                          {label}
                        </span>
                      ))}
                    </div>
                  </CardContent>

                  <CardFooter className="pt-4 pb-5 border-t border-border/50">
                    <Button
                      type="button"
                      onClick={onClose}
                      className="w-full h-11"
                    >
                      Close
                    </Button>
                  </CardFooter>
                </Card>
              </motion.div>
            ) : (
              // ---- Full permissions editor (non-admin users) ----
              <motion.div
                initial={{ opacity: 0, scale: 0.95, y: 10 }}
                animate={{ opacity: 1, scale: 1, y: 0 }}
                exit={{ opacity: 0, scale: 0.95, y: 10 }}
                className="w-full max-w-2xl sm:max-w-3xl lg:max-w-4xl pointer-events-auto"
              >
                <Card className="border-primary/20 bg-card shadow-2xl overflow-hidden">
                  <CardHeader className="relative pb-4">
                    <Button
                      variant="ghost"
                      size="icon"
                      onClick={onClose}
                      className="absolute right-4 top-4 rounded-full h-8 w-8 text-muted-foreground hover:text-foreground"
                    >
                      <X className="h-4 w-4" />
                    </Button>
                    <CardTitle className="flex items-center gap-2.5 text-xl">
                      <Lock className="w-5 h-5 text-primary" />
                      Manage Permissions
                    </CardTitle>
                    <CardDescription>
                      Assign node, metric, and service permissions for{" "}
                      <span className="font-semibold text-foreground">
                        @{user.username}
                      </span>
                    </CardDescription>
                  </CardHeader>

                  <form onSubmit={handleSubmit}>
                    <CardContent className="space-y-6 pt-2 max-h-[350px] sm:max-h-[400px] lg:max-h-[470px] overflow-y-auto pr-2">
                      {isFetching ? (
                        <div className="flex items-center justify-center py-12">
                          <div className="w-6 h-6 border-2 border-primary border-t-transparent rounded-full animate-spin" />
                        </div>
                      ) : nodesWithServices.length === 0 ? (
                        <div className="text-center py-8 text-muted-foreground">
                          <AlertCircle className="w-8 h-8 mx-auto mb-2 opacity-50" />
                          <p className="text-sm">No nodes available</p>
                        </div>
                      ) : (
                        nodesWithServices.map((node) => {
                          const nodePerms = form[node.node_id];

                          if (!nodePerms) {
                            return null;
                          }

                          return (
                            <div
                              key={node.node_id}
                              className="p-4 rounded-xl border border-border bg-muted/30 space-y-4"
                            >
                              {/* Node Header with Full Access Toggle */}
                              <div className="flex items-center justify-between">
                                <div>
                                  <h4 className="font-semibold text-foreground">
                                    {node.node_name}
                                  </h4>
                                </div>
                                <label className="flex items-center gap-2.5 cursor-pointer">
                                  <Checkbox
                                    checked={nodePerms.full_permission}
                                    onCheckedChange={(checked) =>
                                      handleFullPermissionToggle(
                                        node.node_id,
                                        checked as boolean,
                                      )
                                    }
                                    className="h-5 w-5"
                                  />
                                  <span className="text-sm font-medium text-foreground/80 whitespace-nowrap">
                                    Full Access
                                  </span>
                                </label>
                              </div>

                              {/* Metrics & Services (only if not full access) */}
                              {!nodePerms.full_permission && (
                                <div className="space-y-4 pt-2 border-t border-border/50">
                                  {/* Metrics Section */}
                                  <div>
                                    <p className="text-xs font-semibold uppercase tracking-wider text-muted-foreground mb-3">
                                      📊 Metrics
                                    </p>
                                    <div className="grid grid-cols-3 gap-3">
                                      {(
                                        ["RAM", "CPU", "DISK"] as MetricType[]
                                      ).map((metric) => (
                                        <label
                                          key={metric}
                                          className="flex items-center gap-2 cursor-pointer p-2.5 rounded-lg border border-border/50 hover:bg-primary/5 transition-colors"
                                        >
                                          <Checkbox
                                            checked={nodePerms.metrics[metric]}
                                            onCheckedChange={(checked) =>
                                              handleMetricChange(
                                                node.node_id,
                                                metric,
                                                checked as boolean,
                                              )
                                            }
                                            className="h-4 w-4"
                                          />
                                          <span className="text-sm font-medium text-foreground/80">
                                            {metric}
                                          </span>
                                        </label>
                                      ))}
                                    </div>
                                  </div>

                                  {/* Services Section */}
                                  {node.services.length > 0 && (
                                    <div>
                                      <p className="text-xs font-semibold uppercase tracking-wider text-muted-foreground mb-3">
                                        ⚙️ Services ({node.services.length})
                                      </p>
                                      <div className="grid grid-cols-2 gap-2.5 sm:grid-cols-3">
                                        {node.services.map((service) => (
                                          <label
                                            key={service.id}
                                            className="flex items-center gap-2 cursor-pointer p-2.5 rounded-lg border border-border/50 hover:bg-primary/5 transition-colors"
                                          >
                                            <Checkbox
                                              checked={nodePerms.services.includes(
                                                service.id,
                                              )}
                                              onCheckedChange={() =>
                                                handleServiceToggle(
                                                  node.node_id,
                                                  service.id,
                                                )
                                              }
                                              className="h-4 w-4"
                                            />
                                            <span className="text-xs font-medium text-foreground/80 truncate">
                                              {service.service_name}
                                            </span>
                                          </label>
                                        ))}
                                      </div>
                                    </div>
                                  )}
                                </div>
                              )}

                              {/* Full Access Notice */}
                              {nodePerms.full_permission && (
                                <div className="p-3 rounded-lg bg-primary/10 border border-primary/20 text-xs text-primary">
                                  ✓ User has full access to all metrics and
                                  services
                                </div>
                              )}
                            </div>
                          );
                        })
                      )}
                    </CardContent>

                    <CardFooter className="pt-4 pb-5 flex gap-3 border-t border-border/50">
                      <Button
                        type="button"
                        variant="outline"
                        onClick={onClose}
                        className="flex-1 h-11"
                        disabled={isLoading || isFetching}
                      >
                        Cancel
                      </Button>
                      <Button
                        type="submit"
                        className="flex-[1] h-11 bg-gradient-to-r from-primary to-blue-600 hover:opacity-90 transition-all font-bold shadow-md"
                        disabled={isLoading || isFetching}
                      >
                        {isLoading ? (
                          <div className="flex items-center gap-2">
                            <div className="w-4 h-4 border-2 border-background border-t-transparent rounded-full animate-spin" />
                            Assigning...
                          </div>
                        ) : (
                          <div className="flex items-center gap-2">
                            <CheckCircle2 className="w-4 h-4" />
                            Assign Permissions
                          </div>
                        )}
                      </Button>
                    </CardFooter>
                  </form>
                </Card>
              </motion.div>
            )}
          </div>
        </>
      )}
    </AnimatePresence>
  );
}