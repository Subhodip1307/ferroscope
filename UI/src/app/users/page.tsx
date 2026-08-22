"use client";

import { useState, useEffect, useMemo, useRef } from "react";
import { useRouter } from "next/navigation";
import { motion, AnimatePresence } from "framer-motion";
import { Header } from "@/components/dashboard/Header";
import { UserFormModal } from "@/components/users/UserFormModal";
import { DeleteUserModal } from "@/components/users/DeleteUserModal";
import { api } from "@/lib/api";
import type { UserAccessControlItem, UserPermissionsResponse } from "@/types";
import { toast } from "sonner";
import {
  Users,
  Search,
  RefreshCw,
  UserPlus,
  Pencil,
  Trash2,
  Mail,
  Calendar,
  ShieldCheck,
  UserCheck,
  MailWarning,
  Sparkles,
  ArrowLeft,
  ChevronRight,
  Filter,
  User,
  Lock,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import Link from "next/link";
import { formatDateTime } from "@/lib/utils";
import { PermissionsModal } from "@/components/users/PermissionsModal";

export default function UsersPage() {
  const router = useRouter();
  const [users, setUsers] = useState<UserAccessControlItem[]>([]);
  const [userModalMode, setUserModalMode] = useState<"create" | "edit" | null>(
    null,
  );
  const [userToEdit, setUserToEdit] = useState<UserAccessControlItem | null>(
    null,
  );
  const [loading, setLoading] = useState(true);
  const [searchQuery, setSearchQuery] = useState("");
  const [filterEmailStatus, setFilterEmailStatus] = useState<
    "all" | "has_email" | "no_email"
  >("all");

  const [userPermissionsMap, setUserPermissionsMap] = useState<
    Map<number, UserPermissionsResponse>
  >(new Map());

  const [permissionsUser, setPermissionsUser] =
    useState<UserAccessControlItem | null>(null);

  // Snapshot of last server-confirmed list for revert on API failure
  const usersSnapshot = useRef<UserAccessControlItem[]>([]);

  const [deletingUser, setDeletingUser] =
    useState<UserAccessControlItem | null>(null);

  useEffect(() => {
    const token = localStorage.getItem("ferro_token");
    if (!token) {
      router.push("/login");
      return;
    }
    fetchUsers();
  }, []);

  useEffect(() => {
    if (users.length > 0) {
      fetchAllUserPermissions();
    }
  }, [users]);

  const fetchAllUserPermissions = async () => {
    const permissionsMap = new Map<number, UserPermissionsResponse>();

    for (const user of users) {
      try {
        const perms = await api.getUserPermissions(user.id);
        if (perms) {
          permissionsMap.set(user.id, perms);
        }
      } catch (error) {
        console.error(`Error fetching permissions for user ${user.id}:`, error);
      }
    }

    setUserPermissionsMap(permissionsMap);
  };

  const fetchUsers = async () => {
    try {
      setLoading(true);
      const userList = await api.getAllUsers();
      setUsers(userList);
      usersSnapshot.current = userList;
    } catch (error: any) {
      console.error("Error fetching users:", error);
      toast.error(error.message || "Failed to load users");
    } finally {
      setLoading(false);
    }
  };

  const handleRefresh = () => {
    fetchUsers();
    toast.success("User list refreshed", { closeButton: true });
  };

  // ── Optimistic update handlers ────────────────────────────────────────────
  const handleOptimisticEdit = (updated: UserAccessControlItem) => {
    usersSnapshot.current = users; // snapshot before mutation
    setUsers((prev) => prev.map((u) => (u.id === updated.id ? updated : u)));
  };

  const handleOptimisticDelete = (userId: number) => {
    usersSnapshot.current = users; // snapshot before mutation
    setUsers((prev) => prev.filter((u) => u.id !== userId));
    setDeletingUser(null);
  };

  const handleRevert = () => {
    setUsers(usersSnapshot.current);
  };

  // Filtered users calculation
  const filteredUsers = useMemo(() => {
    return users.filter((user) => {
      const q = searchQuery.toLowerCase().trim();
      const matchesSearch =
        !q ||
        user.username.toLowerCase().includes(q) ||
        (user.email && user.email.toLowerCase().includes(q)) ||
        user.id.toString().includes(q);

      const matchesEmail =
        filterEmailStatus === "all" ||
        (filterEmailStatus === "has_email" && !!user.email) ||
        (filterEmailStatus === "no_email" && !user.email);

      return matchesSearch && matchesEmail;
    });
  }, [users, searchQuery, filterEmailStatus]);

  // Statistics
  const stats = useMemo(() => {
    const total = users.length;
    const withEmail = users.filter((u) => !!u.email).length;
    const withoutEmail = total - withEmail;
    const admins = users.filter((u) => u.is_admin).length;
    return { total, withEmail, withoutEmail, admins };
  }, [users]);

  return (
    <div className="min-h-screen bg-linear-to-br from-background via-background to-muted/20">
      <Header onRefresh={handleRefresh} isLoading={loading} />

      <main className="container mx-auto px-4 py-8 max-w-7xl">
        {/* Breadcrumb / Top Bar */}
        <motion.div
          key="users-breadcrumb"
          initial={{ opacity: 0, y: -10 }}
          animate={{ opacity: 1, y: 0 }}
          className="flex items-center gap-2 text-xs text-muted-foreground mb-6"
        >
          <Link
            href="/"
            className="flex items-center gap-1 hover:text-foreground transition-colors"
          >
            <ArrowLeft className="w-3.5 h-3.5" />
            Dashboard
          </Link>
          <ChevronRight className="w-3 h-3 text-muted-foreground/50" />
          <span className="text-foreground font-semibold">User Management</span>
        </motion.div>

        {/* Hero Header */}
        <motion.div
          key="users-hero"
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          className="flex flex-col md:flex-row md:items-center justify-between gap-4 mb-8 bg-card/60 border border-border/80 backdrop-blur-md p-6 rounded-2xl shadow-xs"
        >
          <div className="space-y-1">
            <div className="flex items-center gap-2.5">
              <div className="p-2.5 rounded-xl bg-primary/10 text-primary border border-primary/20">
                <Users className="w-6 h-6" />
              </div>
              <h1 className="text-3xl font-extrabold tracking-tight bg-linear-to-r from-foreground via-foreground to-muted-foreground bg-clip-text text-transparent">
                User Directory & Access
              </h1>
            </div>
            <p className="text-sm text-muted-foreground">
              Manage all registered users, edit profile details, and maintain
              access permissions across Ferroscope.
            </p>
          </div>

          <div className="flex items-center gap-3 self-start md:self-auto">
            <Button
              variant="outline"
              size="sm"
              onClick={handleRefresh}
              disabled={loading}
              className="gap-2 h-10 px-4 rounded-xl border-border"
            >
              <RefreshCw
                className={`w-4 h-4 ${loading ? "animate-spin" : ""}`}
              />
              <span>Refresh</span>
            </Button>

            {/* Create User Button (Placeholder per prompt requirement) */}
            <div className="relative group">
              <Button
                onClick={() => {
                  setUserModalMode("create");
                  setUserToEdit(null);
                }}
                className="gap-2 h-10 px-4 rounded-xl bg-linear-to-r from-primary to-blue-600 hover:opacity-90 transition-all font-bold shadow-md text-primary-foreground"
              >
                <UserPlus className="w-4 h-4" />
                <span>Create User</span>
              </Button>
            </div>
          </div>
        </motion.div>

        {/* Stats Overview */}
        <motion.div
          key="users-stats"
          initial={{ opacity: 0, y: 15 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-8"
        >
          <div className="p-5 rounded-2xl bg-card border border-border shadow-xs hover:border-primary/30 transition-all flex items-center justify-between">
            <div>
              <p className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
                Total Users
              </p>
              <h3 className="text-3xl font-black mt-1 text-foreground">
                {loading ? "..." : stats.total}
              </h3>
            </div>
            <div className="w-12 h-12 rounded-xl bg-blue-500/10 text-blue-500 flex items-center justify-center border border-blue-500/20">
              <UserCheck className="w-6 h-6" />
            </div>
          </div>

          <div className="p-5 rounded-2xl bg-card border border-border shadow-xs hover:border-primary/30 transition-all flex items-center justify-between">
            <div>
              <p className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
                Admins
              </p>
              <h3 className="text-3xl font-black mt-1 text-violet-500">
                {loading ? "..." : stats.admins}
              </h3>
            </div>
            <div className="w-12 h-12 rounded-xl bg-violet-500/10 text-violet-500 flex items-center justify-center border border-violet-500/20">
              <ShieldCheck className="w-6 h-6" />
            </div>
          </div>

          <div className="p-5 rounded-2xl bg-card border border-border shadow-xs hover:border-primary/30 transition-all flex items-center justify-between">
            <div>
              <p className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
                Email Linked
              </p>
              <h3 className="text-3xl font-black mt-1 text-emerald-500">
                {loading ? "..." : stats.withEmail}
              </h3>
            </div>
            <div className="w-12 h-12 rounded-xl bg-emerald-500/10 text-emerald-500 flex items-center justify-center border border-emerald-500/20">
              <Mail className="w-6 h-6" />
            </div>
          </div>

          <div className="p-5 rounded-2xl bg-card border border-border shadow-xs hover:border-primary/30 transition-all flex items-center justify-between">
            <div>
              <p className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
                No Email Linked
              </p>
              <h3 className="text-3xl font-black mt-1 text-amber-500">
                {loading ? "..." : stats.withoutEmail}
              </h3>
            </div>
            <div className="w-12 h-12 rounded-xl bg-amber-500/10 text-amber-500 flex items-center justify-center border border-amber-500/20">
              <MailWarning className="w-6 h-6" />
            </div>
          </div>
        </motion.div>

        {/* Search & Filter Toolbar */}
        <motion.div
          key="users-toolbar"
          initial={{ opacity: 0, y: 15 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.15 }}
          className="flex flex-col sm:flex-row items-center justify-between gap-4 mb-6"
        >
          <div className="relative w-full sm:w-80">
            <Search className="absolute left-3.5 top-3 w-4 h-4 text-muted-foreground" />
            <Input
              placeholder="Search by username, email, ID..."
              className="pl-10 h-10 bg-card border-border rounded-xl focus:ring-2 focus:ring-primary/20 transition-all text-sm"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
            />
            {searchQuery && (
              <button
                onClick={() => setSearchQuery("")}
                className="absolute right-3 top-2.5 text-xs text-muted-foreground hover:text-foreground"
              >
                Clear
              </button>
            )}
          </div>

          {/* <div className="flex items-center gap-2 w-full sm:w-auto">
            <Filter className="w-4 h-4 text-muted-foreground shrink-0" />
            <div className="flex bg-card p-1 rounded-xl border border-border text-xs w-full sm:w-auto">
              <button
                onClick={() => setFilterEmailStatus("all")}
                className={`px-3 py-1.5 rounded-lg transition-all font-medium ${
                  filterEmailStatus === "all"
                    ? "bg-primary text-primary-foreground font-semibold shadow-xs"
                    : "text-muted-foreground hover:text-foreground"
                }`}
              >
                All ({stats.total})
              </button>
              <button
                onClick={() => setFilterEmailStatus("has_email")}
                className={`px-3 py-1.5 rounded-lg transition-all font-medium ${
                  filterEmailStatus === "has_email"
                    ? "bg-primary text-primary-foreground font-semibold shadow-xs"
                    : "text-muted-foreground hover:text-foreground"
                }`}
              >
                With Email
              </button>
              <button
                onClick={() => setFilterEmailStatus("no_email")}
                className={`px-3 py-1.5 rounded-lg transition-all font-medium ${
                  filterEmailStatus === "no_email"
                    ? "bg-primary text-primary-foreground font-semibold shadow-xs"
                    : "text-muted-foreground hover:text-foreground"
                }`}
              >
                No Email
              </button>
            </div>
          </div> */}
        </motion.div>

        {/* User Data Table / Cards */}
        <AnimatePresence mode="wait">
          <motion.div
            key={
              loading
                ? "users-loading"
                : `users-table-${filterEmailStatus}-${searchQuery}`
            }
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -20 }}
            transition={{ delay: 0.2 }}
          >
            {loading ? (
              <div className="bg-card border border-border rounded-2xl p-12 text-center space-y-4 shadow-xs">
                <div className="w-12 h-12 border-4 border-primary border-t-transparent rounded-full animate-spin mx-auto" />
                <p className="text-muted-foreground text-sm font-medium">
                  Fetching user directory...
                </p>
              </div>
            ) : filteredUsers.length === 0 ? (
              <div className="bg-card border border-border rounded-2xl p-12 text-center space-y-3 shadow-xs">
                <div className="w-16 h-16 rounded-full bg-muted/50 flex items-center justify-center mx-auto text-muted-foreground">
                  <Users className="w-8 h-8" />
                </div>
                <h3 className="text-lg font-semibold">No users found</h3>
                <p className="text-sm text-muted-foreground max-w-md mx-auto">
                  {searchQuery
                    ? `No user matches search term "${searchQuery}".`
                    : "No other user accounts found in the database."}
                </p>
                {searchQuery && (
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => setSearchQuery("")}
                    className="mt-2"
                  >
                    Clear Search Filter
                  </Button>
                )}
              </div>
            ) : (
              <div className="bg-card border border-border rounded-2xl shadow-sm overflow-hidden">
                <div className="overflow-x-auto">
                  <table className="w-full text-left border-collapse text-sm">
                    <thead>
                      <tr className="border-b border-border/80 bg-muted/30 text-muted-foreground text-xs uppercase font-semibold tracking-wider">
                        <th className="py-3.5 px-5">User</th>
                        <th className="py-3.5 px-5">Role</th>
                        <th className="py-3.5 px-5">Email</th>
                        <th className="py-3.5 px-5">Joined Date</th>
                        <th className="py-3.5 px-5 text-right">Actions</th>
                      </tr>
                    </thead>
                    <tbody className="divide-y divide-border/60">
                      {filteredUsers.map((user) => (
                        <motion.tr
                          key={user.id}
                          layout
                          initial={{ opacity: 0 }}
                          animate={{ opacity: 1 }}
                          exit={{ opacity: 0 }}
                          className="hover:bg-muted/40 transition-colors group"
                        >
                          {/* User ID & Username */}
                          <td className="py-4 px-5">
                            <div className="flex items-center gap-3">
                              <div className="w-10 h-10 rounded-full bg-gradient-to-tr from-primary/20 to-blue-500/20 text-primary font-bold flex items-center justify-center border border-primary/20 shrink-0">
                                {user.username.charAt(0).toUpperCase()}
                              </div>
                              <div>
                                <div className="flex items-center gap-2">
                                  <span className="font-semibold text-foreground text-base">
                                    {user.username}
                                  </span>
                                </div>
                              </div>
                            </div>
                          </td>
                          <td className="py-4 px-5">
                            {user.is_admin ? (
                              <span className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-semibold bg-violet-500/10 text-violet-500 border border-violet-500/20">
                                <ShieldCheck className="w-3.5 h-3.5" />
                                Admin
                              </span>
                            ) : (
                              <span className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-semibold bg-muted text-muted-foreground border border-border">
                                <User className="w-3.5 h-3.5" />
                                User
                              </span>
                            )}
                          </td>

                          {/* Email */}
                          <td className="py-4 px-5">
                            {user.email ? (
                              <div className="flex items-center gap-2 text-foreground font-mono text-xs">
                                <Mail className="w-3.5 h-3.5 text-emerald-500 shrink-0" />
                                <span>{user.email}</span>
                              </div>
                            ) : (
                              <span className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium bg-amber-500/10 text-amber-500 border border-amber-500/20">
                                No email configured
                              </span>
                            )}
                          </td>

                          <td className="py-4 px-5">
                            {formatDateTime(user.joined_date)}
                          </td>

                          {/* Action Buttons */}
                          <td className="py-4 px-5 text-right">
                            <div className="flex items-center justify-end gap-2">
                              <Button
                                variant="ghost"
                                size="sm"
                                onClick={() => {
                                  const userWithPerms: UserAccessControlItem = {
                                    ...user,
                                    permissions: userPermissionsMap.get(
                                      user.id,
                                    ),
                                  };
                                  setPermissionsUser(userWithPerms);
                                }}
                                className="h-9 px-3 gap-1.5 text-xs font-medium hover:bg-violet-500/10 hover:text-violet-500 transition-colors"
                              >
                                <Lock className="w-3.5 h-3.5" />
                                <span>Permissions</span>
                              </Button>
                              <Button
                                variant="ghost"
                                size="sm"
                                onClick={() => {
                                  setUserModalMode("edit");
                                  setUserToEdit(user);
                                }}
                                className="h-9 px-3 gap-1.5 text-xs font-medium hover:bg-primary/10 hover:text-primary transition-colors"
                              >
                                <Pencil className="w-3.5 h-3.5" />
                                <span>Edit</span>
                              </Button>

                              <Button
                                variant="ghost"
                                size="sm"
                                onClick={() => setDeletingUser(user)}
                                className="h-9 px-3 gap-1.5 text-xs font-medium text-destructive hover:bg-destructive/10 hover:text-destructive transition-colors"
                              >
                                <Trash2 className="w-3.5 h-3.5" />
                                <span>Delete</span>
                              </Button>
                            </div>
                          </td>
                        </motion.tr>
                      ))}
                    </tbody>
                  </table>
                </div>

                <div className="px-5 py-3 border-t border-border/80 bg-muted/20 flex items-center justify-between text-xs text-muted-foreground">
                  <span>
                    Showing {filteredUsers.length} of {users.length} users
                  </span>
                  <span className="font-mono text-[11px]">
                    Ferroscope Access Control
                  </span>
                </div>
              </div>
            )}
          </motion.div>
        </AnimatePresence>
      </main>

      <UserFormModal
        isOpen={userModalMode !== null}
        mode={userModalMode || "create"}
        user={userToEdit}
        onClose={() => {
          setUserModalMode(null);
          setUserToEdit(null);
        }}
        onSuccess={fetchUsers}
        onOptimisticUpdate={handleOptimisticEdit}
        onRevert={handleRevert}
      />

      <PermissionsModal
        isOpen={!!permissionsUser}
        user={permissionsUser}
        onClose={() => setPermissionsUser(null)}
        onSuccess={fetchUsers}
      />

      {/* Delete User Confirmation Modal */}
      <DeleteUserModal
        isOpen={!!deletingUser}
        user={deletingUser}
        onClose={() => setDeletingUser(null)}
        onOptimisticDelete={handleOptimisticDelete}
        onRevert={handleRevert}
      />
    </div>
  );
}
