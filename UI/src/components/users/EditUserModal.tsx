"use client";

import { useState, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { PasswordInput } from "@/components/ui/password-input";
import { Label } from "@/components/ui/label";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { User, Mail, Lock, X, CheckCircle2, UserCog } from "lucide-react";
import { toast } from "sonner";
import { api } from "@/lib/api";
import type { UserAccessControlItem } from "@/types";

interface EditUserModalProps {
  isOpen: boolean;
  user: UserAccessControlItem | null;
  onClose: () => void;
  /** Called immediately with the optimistically updated user, before the API resolves */
  onOptimisticUpdate: (updated: UserAccessControlItem) => void;
  /** Called after API fails so the page can revert to the original list */
  onRevert: () => void;
}

export function EditUserModal({
  isOpen,
  user,
  onClose,
  onOptimisticUpdate,
  onRevert,
}: EditUserModalProps) {
  const [username, setUsername] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    if (user) {
      setUsername(user.username || "");
      setEmail(user.email || "");
      setPassword("");
    }
  }, [user]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!user) return;

    if (!username.trim()) {
      toast.error("Username is required");
      return;
    }

    const finalEmail = email.trim() || null;
    const finalPassword = password.trim() || null;

    // ── Optimistic update: close modal & patch UI immediately ──
    const optimisticUser: UserAccessControlItem = {
      ...user,
      username: username.trim(),
      email: finalEmail,
    };
    onOptimisticUpdate(optimisticUser);
    onClose();

    setIsLoading(true);
    try {
      const success = await api.editUserDetails({
        id: user.id,
        username: username.trim(),
        email: finalEmail,
        password: finalPassword,
      });

      if (success) {
        toast.success(`User "@${username.trim()}" updated successfully!`);
      } else {
        toast.error("Failed to update user details — reverting.");
        onRevert();
      }
    } catch (err: any) {
      console.error("Error editing user:", err);
      toast.error(err.message || "An error occurred while updating user — reverting.");
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
          <div className="fixed inset-0 flex items-center justify-center z-[101] p-4 pointer-events-none overflow-y-auto">
            <motion.div
              initial={{ opacity: 0, scale: 0.95, y: 10 }}
              animate={{ opacity: 1, scale: 1, y: 0 }}
              exit={{ opacity: 0, scale: 0.95, y: 10 }}
              className="w-full max-w-md pointer-events-auto"
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
                    <UserCog className="w-5 h-5 text-primary" />
                    Edit User Details
                  </CardTitle>
                  <CardDescription>
                    Update details for{" "}
                    <span className="font-semibold text-foreground">
                      {user.username}
                    </span>{" "}
                   
                  </CardDescription>
                </CardHeader>

                <form onSubmit={handleSubmit}>
                  <CardContent className="space-y-4 pt-2">
                    {/* Username */}
                    <div className="space-y-2">
                      <Label
                        htmlFor="edit-username"
                        className="text-sm font-semibold text-foreground/80"
                      >
                        Username <span className="text-destructive">*</span>
                      </Label>
                      <div className="relative">
                        <User className="absolute left-3 top-3 w-4 h-4 text-muted-foreground" />
                        <Input
                          id="edit-username"
                          placeholder="Username"
                          className="pl-10 h-11 bg-background/50 border-primary/10 transition-all"
                          value={username}
                          onChange={(e) => setUsername(e.target.value)}
                          required
                        />
                      </div>
                    </div>

                    {/* Email */}
                    <div className="space-y-2">
                      <Label
                        htmlFor="edit-email"
                        className="text-sm font-semibold text-foreground/80"
                      >
                        Email Address{" "}
                        <span className="text-xs font-normal text-muted-foreground">
                          (optional)
                        </span>
                      </Label>
                      <div className="relative">
                        <Mail className="absolute left-3 top-3 w-4 h-4 text-muted-foreground" />
                        <Input
                          id="edit-email"
                          type="email"
                          placeholder="user@example.com"
                          className="pl-10 h-11 bg-background/50 border-primary/10 transition-all"
                          value={email}
                          onChange={(e) => setEmail(e.target.value)}
                        />
                      </div>
                    </div>

                    {/* Password */}
                    <div className="space-y-2">
                      <Label
                        htmlFor="edit-password"
                        className="text-sm font-semibold text-foreground/80"
                      >
                        New Password{" "}
                        <span className="text-xs font-normal text-muted-foreground">
                          (leave blank to keep existing)
                        </span>
                      </Label>
                      <div className="relative">
                        <Lock className="absolute left-3 top-3 w-4 h-4 text-muted-foreground z-10" />
                        <PasswordInput
                          id="edit-password"
                          placeholder="Enter new password to overwrite"
                          className="pl-10 h-11 bg-background/50 border-primary/10 transition-all font-mono"
                          value={password}
                          onChange={(e) => setPassword(e.target.value)}
                        />
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
                      type="submit"
                      className="flex-[2] h-11 bg-gradient-to-r from-primary to-blue-600 hover:opacity-90 transition-all font-bold shadow-md"
                      disabled={isLoading}
                    >
                      {isLoading ? (
                        <div className="flex items-center gap-2">
                          <div className="w-4 h-4 border-2 border-background border-t-transparent rounded-full animate-spin" />
                          Saving...
                        </div>
                      ) : (
                        <div className="flex items-center gap-2">
                          <CheckCircle2 className="w-4 h-4" />
                          Save Changes
                        </div>
                      )}
                    </Button>
                  </CardFooter>
                </form>
              </Card>
            </motion.div>
          </div>
        </>
      )}
    </AnimatePresence>
  );
}
