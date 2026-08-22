"use client";

import { useState, useEffect } from "react";
import { motion } from "framer-motion";
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
import {
  User,
  Mail,
  Lock,
  X,
  UserPlus,
  UserCog,
  ShieldCheck,
  CheckCircle2,
} from "lucide-react";
import { toast } from "sonner";
import { api } from "@/lib/api";
import type { UserAccessControlItem } from "@/types";

interface UserFormModalProps {
  isOpen: boolean;
  mode: "create" | "edit";
  user: UserAccessControlItem | null;
  onClose: () => void;
  onSuccess: () => void;
  onOptimisticUpdate?: (updated: UserAccessControlItem) => void;
  onRevert?: () => void;
}

interface UserForm {
  username: string;
  email: string;
  password: string;
  is_admin: boolean;
}

const DEFAULT_FORM: UserForm = {
  username: "",
  email: "",
  password: "",
  is_admin: false,
};

export function UserFormModal({
  isOpen,
  mode,
  user,
  onClose,
  onSuccess,
  onOptimisticUpdate,
  onRevert,
}: UserFormModalProps) {
  const [form, setForm] = useState<UserForm>(DEFAULT_FORM);
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    if (mode === "edit" && user) {
      setForm({
        username: user.username || "",
        email: user.email || "",
        password: "",
        is_admin: user.is_admin,
      });
    } else if (mode === "create") {
      setForm(DEFAULT_FORM);
    }
  }, [mode, user, isOpen]);

  const resetAndClose = (): void => {
    setForm(DEFAULT_FORM);
    onClose();
  };

  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

  const handleSubmit = async (
    e: React.FormEvent<HTMLFormElement>,
  ): Promise<void> => {
    e.preventDefault();

    if (!form.username.trim()) {
      toast.error("Username is required");
      return;
    }
    if (form.email.trim() && !emailRegex.test(form.email.trim())) {
      toast.error("Please enter a valid email address");
      return;
    }
    if (mode === "create" && !form.password.trim()) {
      toast.error("Password is required");
      return;
    }

    setIsLoading(true);
    try {
      if (mode === "create") {
        const success = await api.createUser({
          username: form.username.trim(),
          email: form.email.trim() || null,
          password: form.password,
          is_admin: form.is_admin,
        });

        if (success) {
          toast.success(
            `User "@${form.username.trim()}" created successfully!`,
          );
          resetAndClose();
          onSuccess();
        }
      } else if (mode === "edit" && user) {
        const optimisticUser: UserAccessControlItem = {
          ...user,
          username: form.username.trim(),
          email: form.email.trim() || null,
          is_admin: form.is_admin,
        };
        onOptimisticUpdate?.(optimisticUser);
        onClose();

        const finalPassword = form.password.trim() || null;
        const success = await api.editUserDetails({
          id: user.id,
          username: form.username.trim(),
          email: form.email.trim() || null,
          is_admin: form.is_admin,
          password: finalPassword,
        });

        if (success) {
          toast.success(
            `User "@${form.username.trim()}" updated successfully!`,
          );
        } else {
          toast.error("Failed to update user details — reverting.");
          onRevert?.();
        }
      }
    } catch (error: unknown) {
      const err = error as Error & { status?: number };

      if (mode === "create" && err?.status === 409) {
        toast.error("Username already exists. Please choose a different one.");
      } else {
        console.error("Error:", error);
        const action = mode === "create" ? "creating" : "updating";
        const message =
          err instanceof Error
            ? err.message
            : `An error occurred while ${action} user`;
        toast.error(message);
        if (mode === "edit") {
          onRevert?.();
        }
      }
    } finally {
      setIsLoading(false);
    }
  };

  if (!isOpen) {
    return null;
  }

  const isCreate = mode === "create";
  const title = isCreate ? "Create New User" : "Edit User Details";
  const icon = isCreate ? (
    <UserPlus className="w-5 h-5" />
  ) : (
    <UserCog className="w-5 h-5" />
  );
  const description = isCreate
    ? "Add a new user account to Ferroscope. Password must be set on creation."
    : `Update details for @${user?.username}`;
  const submitLabel = isCreate ? "Create User" : "Save Changes";

  return (
    <>
      {/* Backdrop */}
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
        onClick={resetAndClose}
        className="fixed inset-0 bg-black/70 z-100"
      />

      {/* Modal */}
      <div className="fixed inset-0 flex items-center justify-center z-101 p-4 pointer-events-none overflow-y-auto">
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
                onClick={resetAndClose}
                className="absolute right-4 top-4 rounded-full h-8 w-8 text-muted-foreground hover:text-foreground"
              >
                <X className="h-4 w-4" />
              </Button>
              <CardTitle className="flex items-center gap-2.5 text-xl">
                {icon}
                {title}
              </CardTitle>
              <CardDescription>{description}</CardDescription>
            </CardHeader>

            <form onSubmit={handleSubmit}>
              <CardContent className="space-y-4 pt-2">
                {/* Username */}
                <div className="space-y-2">
                  <Label
                    htmlFor="form-username"
                    className="text-sm font-semibold text-foreground/80"
                  >
                    Username <span className="text-destructive">*</span>
                  </Label>
                  <div className="relative">
                    <User className="absolute left-3 top-3 w-4 h-4 text-muted-foreground" />
                    <Input
                      id="form-username"
                      placeholder={isCreate ? "e.g. alice" : "Username"}
                      className="pl-10 h-11 bg-background/50 border-primary/10 transition-all"
                      value={form.username}
                      onChange={(e) =>
                        setForm((f) => ({ ...f, username: e.target.value }))
                      }
                      required
                      autoComplete="off"
                      autoFocus
                    />
                  </div>
                </div>

                {/* Email */}
                <div className="space-y-2">
                  <Label
                    htmlFor="form-email"
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
                      id="form-email"
                      type="email"
                      placeholder="user@example.com"
                      className="pl-10 h-11 bg-background/50 border-primary/10 transition-all"
                      value={form.email}
                      onChange={(e) =>
                        setForm((f) => ({ ...f, email: e.target.value }))
                      }
                      autoComplete="off"
                    />
                  </div>
                </div>

                {/* Password */}
                <div className="space-y-2">
                  <Label
                    htmlFor="form-password"
                    className="text-sm font-semibold text-foreground/80"
                  >
                    {isCreate ? (
                      <>
                        Password <span className="text-destructive">*</span>
                      </>
                    ) : (
                      <>
                        New Password{" "}
                        <span className="text-xs font-normal text-muted-foreground">
                          (leave blank to keep existing)
                        </span>
                      </>
                    )}
                  </Label>
                  <div className="relative">
                    <Lock className="absolute left-3 top-3 w-4 h-4 text-muted-foreground z-10" />
                    <PasswordInput
                      id="form-password"
                      placeholder={
                        isCreate
                          ? "Set a strong password"
                          : "Enter new password to overwrite"
                      }
                      className="pl-10 h-11 bg-background/50 border-primary/10 transition-all font-mono"
                      value={form.password}
                      onChange={(e) =>
                        setForm((f) => ({ ...f, password: e.target.value }))
                      }
                      required={isCreate}
                    />
                  </div>
                </div>

                {/* Admin Toggle */}
                <button
                  type="button"
                  onClick={() =>
                    setForm((f) => ({ ...f, is_admin: !f.is_admin }))
                  }
                  className={`w-full flex items-center justify-between gap-4 p-4 rounded-xl border cursor-pointer select-none transition-all text-left ${
                    form.is_admin
                      ? "border-primary/40 bg-primary/5"
                      : "border-border bg-muted/20 hover:bg-muted/40"
                  }`}
                >
                  <div className="flex items-center gap-3">
                    <div
                      className={`p-2 rounded-lg transition-colors ${
                        form.is_admin
                          ? "bg-primary/15 text-primary"
                          : "bg-muted text-muted-foreground"
                      }`}
                    >
                      <ShieldCheck className="w-4 h-4" />
                    </div>
                    <div>
                      <p className="text-sm font-semibold text-foreground">
                        Grant Admin Privileges
                      </p>
                      <p className="text-xs text-muted-foreground">
                        Admin users have full system access
                      </p>
                    </div>
                  </div>

                  {/* Toggle pill */}
                  <div
                    className={`relative w-10 h-6 rounded-full transition-colors shrink-0 ${
                      form.is_admin ? "bg-primary" : "bg-muted-foreground/30"
                    }`}
                  >
                    <div
                      className={`absolute top-1 w-4 h-4 rounded-full bg-white shadow transition-transform ${
                        form.is_admin ? "translate-x-5" : "translate-x-1"
                      }`}
                    />
                  </div>
                </button>
              </CardContent>

              <CardFooter className="pt-2 pb-5 flex gap-3">
                <Button
                  type="button"
                  variant="outline"
                  onClick={resetAndClose}
                  className="flex-1 h-11"
                  disabled={isLoading}
                >
                  Cancel
                </Button>
                <Button
                  type="submit"
                  className="flex-2 h-11 bg-linear-to-r from-primary to-blue-600 hover:opacity-90 transition-all font-bold shadow-md"
                  disabled={isLoading}
                >
                  {isLoading ? (
                    <div className="flex items-center gap-2">
                      <div className="w-4 h-4 border-2 border-background border-t-transparent rounded-full animate-spin" />
                      {isCreate ? "Creating..." : "Saving..."}
                    </div>
                  ) : (
                    <div className="flex items-center gap-2">
                      {isCreate ? (
                        <UserPlus className="w-4 h-4" />
                      ) : (
                        <CheckCircle2 className="w-4 h-4" />
                      )}
                      {submitLabel}
                    </div>
                  )}
                </Button>
              </CardFooter>
            </form>
          </Card>
        </motion.div>
      </div>
    </>
  );
}
