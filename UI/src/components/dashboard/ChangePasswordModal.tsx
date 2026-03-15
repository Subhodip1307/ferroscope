"use client";

import { useState, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Lock, User, X, CheckCircle2, AlertCircle, Key } from "lucide-react";
import { toast } from "sonner";
import { api } from "@/lib/api";

interface ChangePasswordModalProps {
    isOpen: boolean;
    onClose: () => void;
    onSuccess: (newUsername: string) => void;
    initialUsername?: string;
}

export function ChangePasswordModal({ isOpen, onClose, onSuccess, initialUsername }: ChangePasswordModalProps) {
    const [username, setUsername] = useState(initialUsername || "");
    const [password, setPassword] = useState("");
    const [isLoading, setIsLoading] = useState(false);

    useEffect(() => {
        if (initialUsername) {
            setUsername(initialUsername);
        }
    }, [initialUsername]);

    // Reset form when modal opens/closes
    useEffect(() => {
        if (!isOpen) {
            setPassword("");
        }
    }, [isOpen]);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setIsLoading(true);

        try {
            const success = await api.changePassword({ username, password });

            if (success) {
                toast.success("Credentials updated successfully!");
                onSuccess(username);
                onClose();
            } else {
                toast.error("Conflict: Credentials not updated. Username might be taken.");
            }
        } catch (err) {
            console.error("Change password error:", err);
            toast.error("An error occurred while updating credentials.");
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <AnimatePresence>
            {isOpen && (
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
                                <CardHeader className="relative">
                                    <Button
                                        variant="ghost"
                                        size="icon"
                                        onClick={onClose}
                                        className="absolute right-4 top-4 rounded-full h-8 w-8 text-muted-foreground hover:text-foreground"
                                    >
                                        <X className="h-4 w-4" />
                                    </Button>
                                    <CardTitle className="flex items-center gap-2 text-xl">
                                        <Key className="w-5 h-5 text-primary" />
                                        Security Settings
                                    </CardTitle>
                                    <CardDescription>
                                        Update your username or set a new password.
                                    </CardDescription>
                                </CardHeader>
                                <form onSubmit={handleSubmit}>
                                    <CardContent className="space-y-4 pt-2">
                                        <div className="space-y-2">
                                            <Label htmlFor="modal-username" className="text-sm font-semibold text-foreground/80">
                                                Username
                                            </Label>
                                            <div className="relative">
                                                <User className="absolute left-3 top-3 w-4 h-4 text-muted-foreground" />
                                                <Input
                                                    id="modal-username"
                                                    placeholder="Your current or new username"
                                                    className="pl-10 h-11 bg-background/50 border-primary/10 transition-all"
                                                    value={username}
                                                    onChange={(e) => setUsername(e.target.value)}
                                                    required
                                                />
                                            </div>
                                        </div>

                                        <div className="space-y-2">
                                            <Label htmlFor="modal-password" className="text-sm font-semibold text-foreground/80">
                                                New Password
                                            </Label>
                                            <div className="relative">
                                                <Lock className="absolute left-3 top-3 w-4 h-4 text-muted-foreground" />
                                                <Input
                                                    id="modal-password"
                                                    type="password"
                                                    placeholder="••••••••"
                                                    className="pl-10 h-11 bg-background/50 border-primary/10 transition-all font-mono"
                                                    value={password}
                                                    onChange={(e) => setPassword(e.target.value)}
                                                    required
                                                />
                                            </div>
                                        </div>
                                    </CardContent>
                                    <CardFooter className="pt-2 flex gap-3">
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
                                            className="flex-[2] h-11 bg-gradient-to-r from-primary to-blue-600 hover:opacity-90 transition-all font-bold"
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
                                                    Update
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
