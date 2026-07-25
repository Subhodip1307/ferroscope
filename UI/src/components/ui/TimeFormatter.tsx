"use client";

import React from "react";
import { Clock, Calendar } from "lucide-react";
import { formatDateTime, formatDateShort, formatRelativeTime } from "@/lib/utils";

export interface TimeFormatterProps {
  /** ISO 8601 UTC date string (e.g. "2026-05-14T09:32:11Z") */
  dateString: string | null | undefined;
  /** Format mode: 'relative' (e.g. "2m ago"), 'full' (e.g. "May 14, 2026, 09:32:11 AM"), or 'short' (e.g. "May 14, 2026") */
  mode?: "relative" | "full" | "short" | "both";
  /** Show leading icon (Clock/Calendar) */
  showIcon?: boolean;
  /** Custom CSS class names */
  className?: string;
}

export function TimeFormatter({
  dateString,
  mode = "both",
  showIcon = true,
  className = "",
}: TimeFormatterProps) {
  if (!dateString) {
    return <span className={`text-muted-foreground text-xs ${className}`}>N/A</span>;
  }

  const fullText = formatDateTime(dateString);
  const relativeText = formatRelativeTime(dateString);
  const shortText = formatDateShort(dateString);

  return (
    <span
      className={`inline-flex items-center gap-1.5 font-mono text-xs ${className}`}
      title={`UTC Timestamp: ${dateString} | Local: ${fullText}`}
    >
      {showIcon && (
        <Clock className="w-3.5 h-3.5 text-primary/70 shrink-0" />
      )}
      {mode === "relative" && <span>{relativeText}</span>}
      {mode === "full" && <span>{fullText}</span>}
      {mode === "short" && <span>{shortText}</span>}
      {mode === "both" && (
        <span className="flex items-center gap-1.5">
          <span className="font-semibold text-foreground">{shortText}</span>
          <span className="text-muted-foreground text-[11px]">({relativeText})</span>
        </span>
      )}
    </span>
  );
}
