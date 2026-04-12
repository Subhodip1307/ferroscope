"use client";

import { useState, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Card, CardHeader, CardTitle, CardContent, CardFooter, CardDescription } from "@/components/ui/card";
import { X, Bell, Save, AlertTriangle, Plus, Trash2, Mail, Globe, Settings, Activity } from "lucide-react";
import { toast } from "sonner";
import { api } from "@/lib/api";
import type { EventType, RuleChannel, Rule } from "@/types";

interface CreateRuleModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSuccess?: () => void;
}

export function CreateRuleModal({ isOpen, onClose, onSuccess }: CreateRuleModalProps) {
  const [isLoading, setIsLoading] = useState(false);
  const [name, setName] = useState("");
  const [active, setActive] = useState(true);
  const [eventType, setEventType] = useState<EventType>("CPU");
  const [conditionField, setConditionField] = useState<"Status" | "Value">("Value");
  const [conditionOperator, setConditionOperator] = useState<string>(">");
  const [conditionValue, setConditionValue] = useState<number>(90);
  const [actionChannel, setActionChannel] = useState<RuleChannel>("Webhook");
  const [actionToList, setActionToList] = useState<string[]>([]);
  const [actionToInput, setActionToInput] = useState("");
  const [actionMessage, setActionMessage] = useState("");

  // Set default message based on event type
  useEffect(() => {
    if (!actionMessage || actionMessage.startsWith("Alert:")) {
      setActionMessage(`Alert: ${eventType} threshold reached!`);
    }
  }, [eventType]);

  // Handle condition field changes
  useEffect(() => {
    if (conditionField === "Status") {
      setConditionOperator("=");
      setConditionValue(0); // Default to down
    } else {
      setConditionOperator(">");
      setConditionValue(90);
    }
  }, [conditionField]);

  const resetForm = () => {
    setName("");
    setActive(true);
    setEventType("CPU");
    setConditionField("Value");
    setConditionOperator(">");
    setConditionValue(90);
    setActionChannel("Webhook");
    setActionToList([]);
    setActionToInput("");
    setActionMessage("");
  };

  const addTag = () => {
    const val = actionToInput.trim();
    if (val && !actionToList.includes(val)) {
      setActionToList([...actionToList, val]);
      setActionToInput("");
    }
  };

  const removeTag = (index: number) => {
    setActionToList(actionToList.filter((_, i) => i !== index));
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" || e.key === ",") {
      e.preventDefault();
      addTag();
    } else if (e.key === "Backspace" && !actionToInput && actionToList.length > 0) {
      removeTag(actionToList.length - 1);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim()) {
      toast.error("Please enter a name for the rule");
      return;
    }
    if (actionToList.length === 0 && !actionToInput.trim()) {
      toast.error("Please enter at least one destination (URL or Email)");
      return;
    }

    setIsLoading(true);

    try {
      const finalToList = [...actionToList];
      if (actionToInput.trim()) {
        finalToList.push(actionToInput.trim());
      }
      
      const ruleData: Rule = {
        name,
        active,
        event_type: eventType,
        condition: {
          field: conditionField,
          operator: conditionOperator as any,
          value: Number(conditionValue),
        },
        action: {
          channel: actionChannel,
          to: finalToList,
          message: actionMessage,
        },
      };

      const success = await api.createRule(ruleData);

      if (success) {
        toast.success("Notification rule created successfully!", { closeButton: true });
        resetForm();
        onSuccess?.();
        onClose();
      } else {
        toast.error("Failed to create rule. Please try again.", { closeButton: true });
      }
    } catch (error) {
      console.error("Create rule error:", error);
      toast.error("An error occurred while creating the rule.", { closeButton: true });
    } finally {
      setIsLoading(false);
    }
  };

  const inputClasses = "h-11 bg-background/50 border-primary/10 transition-all focus:border-primary/30";
  const selectClasses = "flex h-11 w-full rounded-md border border-primary/10 bg-background/50 px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 transition-all";

  return (
    <AnimatePresence>
      {isOpen && (
        <>
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            onClick={onClose}
            className="fixed inset-0 bg-black/70 z-[100] backdrop-blur-sm"
          />
          <div className="fixed inset-0 flex items-center justify-center z-[101] p-4 pointer-events-none overflow-y-auto">
            <motion.div
              initial={{ opacity: 0, scale: 0.95, y: 10 }}
              animate={{ opacity: 1, scale: 1, y: 0 }}
              exit={{ opacity: 0, scale: 0.95, y: 10 }}
              className="w-full max-w-2xl pointer-events-auto my-8"
            >
              <Card className="w-full p-6 border-primary/20 bg-card shadow-2xl overflow-hidden">
                <div className="flex justify-between items-center mb-6">
                  <div className="flex items-center gap-3">
                    <div className="p-2 rounded-lg bg-primary/10">
                      <Bell className="w-5 h-5 text-primary" />
                    </div>
                    <div>
                      <h2 className="text-xl font-semibold">Create Notification Rule</h2>
                      <p className="text-sm text-muted-foreground">Define system event notifications</p>
                    </div>
                  </div>
                  <Button variant="ghost" size="icon" onClick={onClose} className="rounded-full">
                    <X className="h-4 w-4" />
                  </Button>
                </div>

                <form onSubmit={handleSubmit} className="space-y-6">
                  <div className="max-h-[60vh] overflow-y-auto custom-scrollbar pr-2 space-y-6">
                    {/* Basic Info */}
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                      <div className="space-y-2">
                        <Label htmlFor="rule-name" className="text-sm font-semibold text-foreground/80">Rule Name</Label>
                        <Input
                          id="rule-name"
                          placeholder="e.g., High CPU Alert"
                          className={inputClasses}
                          value={name}
                          onChange={(e) => setName(e.target.value)}
                          required
                        />
                      </div>
                      <div className="space-y-2">
                        <Label htmlFor="event-type" className="text-sm font-semibold text-foreground/80">Event Type</Label>
                        <select
                          id="event-type"
                          className={selectClasses}
                          value={eventType}
                          onChange={(e) => setEventType(e.target.value as EventType)}
                        >
                          <option value="CPU">CPU Usage</option>
                          <option value="RAM">RAM Usage</option>
                          <option value="SERVICE">Service Status</option>
                          <option value="NODE">Node Status</option>
                        </select>
                      </div>
                    </div>

                    <div className="flex items-center gap-3 p-3 bg-primary/5 rounded-lg border border-primary/10">
                      <Settings className="w-4 h-4 text-primary" />
                      <div className="flex-1 flex items-center justify-between">
                        <Label htmlFor="rule-active" className="text-sm font-medium cursor-pointer">Rule is active</Label>
                        <input
                          type="checkbox"
                          id="rule-active"
                          className="w-5 h-5 rounded border-primary/20 text-primary focus:ring-primary transition-all cursor-pointer"
                          checked={active}
                          onChange={(e) => setActive(e.target.checked)}
                        />
                      </div>
                    </div>

                    {/* Condition Section */}
                    <div className="space-y-4 pt-2">
                      <div className="flex items-center gap-2 border-l-2 border-amber-500 pl-3">
                        <Activity className="w-4 h-4 text-amber-500" />
                        <h3 className="text-sm font-bold uppercase tracking-wider text-muted-foreground">Condition</h3>
                      </div>
                      
                      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                        <div className="space-y-2">
                          <Label className="text-xs font-semibold text-muted-foreground">Field</Label>
                          <select
                            className={selectClasses}
                            value={conditionField}
                            onChange={(e) => setConditionField(e.target.value as "Status" | "Value")}
                          >
                            <option value="Value">Numeric Value</option>
                            <option value="Status">Status (Up/Down)</option>
                          </select>
                        </div>

                        <div className="space-y-2">
                          <Label className="text-xs font-semibold text-muted-foreground">Operator</Label>
                          <select
                            className={selectClasses}
                            value={conditionOperator}
                            disabled={conditionField === "Status"}
                            onChange={(e) => setConditionOperator(e.target.value)}
                          >
                            <option value="=">=</option>
                            {conditionField === "Value" && (
                              <>
                                <option value=">">&gt;</option>
                                <option value="<">&lt;</option>
                                <option value=">=">&gt;=</option>
                                <option value="<=">&lt;=</option>
                              </>
                            )}
                          </select>
                        </div>

                        <div className="space-y-2">
                          <Label className="text-xs font-semibold text-muted-foreground">Value</Label>
                          {conditionField === "Status" ? (
                            <select
                              className={selectClasses}
                              value={conditionValue}
                              onChange={(e) => setConditionValue(Number(e.target.value))}
                            >
                              <option value={0}>Down (0)</option>
                              <option value={1}>Up (1)</option>
                            </select>
                          ) : (
                            <Input
                              type="number"
                              className={inputClasses}
                              value={conditionValue}
                              onChange={(e) => setConditionValue(Number(e.target.value))}
                            />
                          )}
                        </div>
                      </div>
                    </div>

                    {/* Action Section */}
                    <div className="space-y-4 pt-2">
                      <div className="flex items-center gap-2 border-l-2 border-blue-500 pl-3">
                        <Globe className="w-4 h-4 text-blue-500" />
                        <h3 className="text-sm font-bold uppercase tracking-wider text-muted-foreground">Action</h3>
                      </div>

                      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div className="space-y-2">
                          <Label className="text-xs font-semibold text-muted-foreground">Channel</Label>
                          <div className="flex gap-2">
                            <Button
                              type="button"
                              variant={actionChannel === "Webhook" ? "default" : "outline"}
                              className="flex-1 gap-2"
                              onClick={() => setActionChannel("Webhook")}
                            >
                              <Globe className="w-4 h-4" />
                              Webhook
                            </Button>
                            <Button
                              type="button"
                              variant={actionChannel === "Email" ? "default" : "outline"}
                              className="flex-1 gap-2"
                              onClick={() => setActionChannel("Email")}
                            >
                              <Mail className="w-4 h-4" />
                              Email
                            </Button>
                          </div>
                        </div>

                        <div className="space-y-2">
                          <Label htmlFor="action-to" className="text-xs font-semibold text-muted-foreground">
                            {actionChannel === "Webhook" ? "Webhook URLs" : "Email Addresses"}
                          </Label>
                          <div 
                            className="max-h-[120px] overflow-y-auto p-1.5 bg-background/50 border border-primary/10 rounded-md flex flex-wrap gap-2 transition-all focus-within:border-primary/30 cursor-text custom-scrollbar"
                            onClick={() => document.getElementById("action-to")?.focus()}
                          >
                            <AnimatePresence>
                              {actionToList.map((tag, idx) => (
                                <motion.div
                                  key={`${tag}-${idx}`}
                                  initial={{ opacity: 0, scale: 0.8 }}
                                  animate={{ opacity: 1, scale: 1 }}
                                  exit={{ opacity: 0, scale: 0.8 }}
                                  className="flex items-center gap-1.5 px-2 py-1 bg-primary/10 border border-primary/20 rounded-md text-sm text-primary"
                                >
                                  <span className="max-w-[150px] truncate">{tag}</span>
                                  <button
                                    type="button"
                                    onClick={() => removeTag(idx)}
                                    className="hover:text-destructive transition-colors"
                                  >
                                    <X className="w-3 h-3" />
                                  </button>
                                </motion.div>
                              ))}
                            </AnimatePresence>
                            <input
                              id="action-to"
                              type="text"
                              className="flex-1 bg-transparent border-none outline-none text-sm min-w-[120px] h-7"
                              placeholder={actionToList.length === 0 ? (actionChannel === "Webhook" ? "https://hooks.slack.com/..." : "admin@example.com") : ""}
                              value={actionToInput}
                              onChange={(e) => setActionToInput(e.target.value)}
                              onKeyDown={handleKeyDown}
                              onBlur={addTag}
                            />
                          </div>
                          <p className="text-[10px] text-muted-foreground mt-1">Press Enter or comma to add multiple</p>
                        </div>
                      </div>

                      <div className="space-y-2">
                        <Label htmlFor="action-message" className="text-sm font-semibold text-foreground/80">Message Body</Label>
                        <textarea
                          id="action-message"
                          rows={3}
                          className="flex min-h-[80px] w-full rounded-md border border-primary/10 bg-background/50 px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 transition-all focus:border-primary/30"
                          placeholder="Message content to include in notification"
                          value={actionMessage}
                          onChange={(e) => setActionMessage(e.target.value)}
                        />
                      </div>
                    </div>
                  </div>

                  <div className="pt-6 border-t border-primary/5 flex gap-3">
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
                      className="flex-[2] h-11 bg-gradient-to-r from-primary to-blue-600 hover:opacity-90 transition-all font-bold shadow-lg shadow-primary/20"
                      disabled={isLoading}
                    >
                      {isLoading ? (
                        <div className="flex items-center gap-2">
                          <div className="w-4 h-4 border-2 border-background border-t-transparent rounded-full animate-spin" />
                          Creating...
                        </div>
                      ) : (
                        <div className="flex items-center gap-2">
                          <Save className="w-4 h-4" />
                          Create Rule
                        </div>
                      )}
                    </Button>
                  </div>
                </form>
              </Card>
            </motion.div>
          </div>
        </>
      )}
    </AnimatePresence>
  );
}
