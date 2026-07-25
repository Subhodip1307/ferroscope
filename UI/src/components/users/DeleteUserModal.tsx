"use client";

import { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Trash2, AlertTriangle, X, User } from "lucide-react";
import { toast } from "sonner";
import { api } from "@/lib/api";
import type { UserAccessControlItem } from "@/types";

interface DeleteUserModalProps {
  isOpen: boolean;
  user: UserAccessControlItem | null;
  onClose: () => void;
  /** Called immediately (before API) to remove the user from the list */
  onOptimisticDelete: (userId: number) => void;
  /** Called if the API call fails so the page can restore the user */
  onRevert: () => void;
}

export function DeleteUserModal({
  isOpen,
  user,
  onClose,
  onOptimisticDelete,
  onRevert,
}: DeleteUserModalProps) {
  const [isLoading, setIsLoading] = useState(false);

  const handleDelete = async () => {
    if (!user) return;

    // ── Optimistic delete: remove from list & close modal instantly ──
    onOptimisticDelete(user.id);
    onClose();

    setIsLoading(true);
    try {
      const success = await api.deleteUser(user.id);
      if (success) {
        toast.success(`User "@${user.username}" deleted successfully.`);
      } else {
        toast.error("Failed to delete user — reverting.");
        onRevert();
      }
    } catch (err: any) {
      console.error("Delete user error:", err);
      toast.error(err.message || "Failed to delete user — reverting.");
      onRevert();
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

          {/* Modal Container */}
          <div className="fixed inset-0 flex items-center justify-center z-[101] p-4 pointer-events-none">
            <motion.div
              initial={{ opacity: 0, scale: 0.95, y: 10 }}
              animate={{ opacity: 1, scale: 1, y: 0 }}
              exit={{ opacity: 0, scale: 0.95, y: 10 }}
              className="w-full max-w-md pointer-events-auto"
            >
              <Card className="border-destructive/30 bg-card shadow-2xl overflow-hidden">
                <CardHeader className="relative pb-3">
                  <Button
                    variant="ghost"
                    size="icon"
                    onClick={onClose}
                    className="absolute right-4 top-4 rounded-full h-8 w-8 text-muted-foreground hover:text-foreground"
                  >
                    <X className="h-4 w-4" />
                  </Button>
                  <div className="w-12 h-12 rounded-full bg-destructive/10 text-destructive flex items-center justify-center mb-2 border border-destructive/20">
                    <AlertTriangle className="w-6 h-6" />
                  </div>
                  <CardTitle className="text-xl">Delete User Account</CardTitle>
                  <CardDescription>
                    This action cannot be undone. Are you sure you want to permanently delete this user account?
                  </CardDescription>
                </CardHeader>

                <CardContent className="pt-2 pb-4">
                  <div className="p-3.5 rounded-xl bg-muted/40 border border-border flex items-center gap-3">
                    <div className="w-10 h-10 rounded-full bg-primary/10 flex items-center justify-center text-primary border border-primary/20 shrink-0">
                      <User className="w-5 h-5" />
                    </div>
                    <div className="min-w-0 flex-1">
                      <p className="text-sm font-semibold truncate text-foreground">
                        {user.username}
                      </p>
                      <p className="text-xs text-muted-foreground truncate">
                        {user.email || "No email linked"}
                      </p>
                    </div>
                  </div>
                </CardContent>

                <CardFooter className="pt-2 pb-5 flex gap-3">
                  <Button
                    type="button"
                    variant="outline"
                    onClick={onClose}
                    className="flex-1 h-11"
                  >
                    Cancel
                  </Button>
                  <Button
                    type="button"
                    onClick={handleDelete}
                    disabled={isLoading}
                    className="flex-[1.5] h-11 bg-destructive hover:bg-destructive/90 text-destructive-foreground font-bold shadow-md transition-all"
                  >
                    {isLoading ? (
                      <div className="flex items-center gap-2">
                        <div className="w-4 h-4 border-2 border-destructive-foreground border-t-transparent rounded-full animate-spin" />
                        Deleting...
                      </div>
                    ) : (
                      <div className="flex items-center gap-2">
                        <Trash2 className="w-4 h-4" />
                        Confirm Delete
                      </div>
                    )}
                  </Button>
                </CardFooter>
              </Card>
            </motion.div>
          </div>
        </>
      )}
    </AnimatePresence>
  );
}
