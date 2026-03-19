"use client";

import { useEffect, useState, useCallback } from "react";
import { motion } from "framer-motion";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  AreaChart,
  Area,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
} from "recharts";
import { api } from "@/lib/api";
import { formatTimestamp, parseRAM } from "@/lib/utils";
import type { RAMData } from "@/types";

interface RAMChartProps {
  nodeId: number;
  nodeName: string;
}

export function RAMChart({ nodeId, nodeName }: RAMChartProps) {
  const [data, setData] = useState<RAMData[]>([]);
  const [loading, setLoading] = useState(true);
  const [isOffline, setIsOffline] = useState(false);

  const fetchHistory = useCallback(async () => {
    try {
      const ramHistory = await api.getRAMHistory(nodeId);
      setData(ramHistory.slice(0, 20));
      setIsOffline(false);
    } catch (error: any) {
      console.error("Error fetching RAM history:", error);
      if (error.message?.includes("503")) {
        setIsOffline(true);
      }
    } finally {
      setLoading(false);
    }
  }, [nodeId]);

  useEffect(() => {
    fetchHistory();

    // Connect to SSE for real-time updates
    const streamUrl = api.getRAMStreamUrl(nodeId);
    const eventSource = new EventSource(streamUrl);

    eventSource.onmessage = (event) => {
      try {
        setIsOffline(false);
        const newData = JSON.parse(event.data);
        const formattedPoint: RAMData = {
          free: newData.free,
          total: newData.total,
          timestamp: newData.timestamp
        };

        setData(prevData => {
          // Keep only last 20 points
          const updated = [formattedPoint, ...prevData];
          return updated.slice(0, 20);
        });
      } catch (err) {
        console.error("Error parsing RAM stream data:", err);
      }
    };

    eventSource.onerror = (err) => {
      setIsOffline(true);
    };

    return () => {
      eventSource.close();
    };
  }, [fetchHistory, nodeId]);

  // ✅ spread first to avoid mutating state array
  const chartData = [...data].reverse().map((item) => ({
    timestamp: item.timestamp,
    time: formatTimestamp(item.timestamp),
    used: parseRAM(item.total) - parseRAM(item.free),
    total: parseRAM(item.total),
  }));

  console.log("chartData:", chartData);

  // ✅ guard against empty array — Math.max(...[]) = -Infinity
  const maxTotal =
    chartData.length > 0 ? Math.max(...chartData.map((d) => d.total), 0) : 16;

  // Smart scaling
  let roundedMax = 16;
  if (maxTotal > 16) {
    const magnitude = Math.pow(10, Math.floor(Math.log10(maxTotal)));
    roundedMax = Math.ceil(maxTotal / magnitude) * magnitude;
  }

  return (
    <motion.div
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      transition={{ duration: 0.5 }}
    >
      <Card>
        <CardHeader>
          <CardTitle className="text-lg">RAM History - {nodeName}</CardTitle>
        </CardHeader>
        <CardContent>
          {loading ? (
            <div className="h-[300px] flex items-center justify-center">
              <div className="animate-pulse text-muted-foreground">
                Loading chart...
              </div>
            </div>
          ) : isOffline ? (
            <div className="h-[300px] flex flex-col items-center justify-center space-y-2">
              <div className="text-destructive font-semibold">Node Offline</div>
              <div className="text-muted-foreground text-sm text-center px-4">
                Wait for the node to come back online for real-time data.
              </div>
            </div>
          ) : chartData.length === 0 ? (
            // ✅ handle empty state
            <div className="h-[300px] flex items-center justify-center">
              <div className="text-muted-foreground text-sm">
                No RAM data available
              </div>
            </div>
          ) : (
            <ResponsiveContainer width="100%" height={300}>
              <AreaChart
                data={chartData}
                margin={{ top: 5, right: 10, left: 10, bottom: 5 }}
              >
                <defs>
                  <linearGradient id="ramGradient" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="#10b981" stopOpacity={0.8} />
                    <stop offset="95%" stopColor="#10b981" stopOpacity={0} />
                  </linearGradient>
                </defs>
                <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
                <XAxis
                  dataKey="timestamp"
                  className="text-xs"
                  stroke="currentColor"
                  tickFormatter={formatTimestamp}
                  minTickGap={30}
                />
                <YAxis
                  domain={[0, roundedMax]}
                  className="text-xs"
                  stroke="currentColor"
                  tickFormatter={(value) => `${Number(value).toFixed(0)} GiB`}
                />
                <Tooltip
                  contentStyle={{
                    backgroundColor: "hsl(var(--background))",
                    border: "1px solid hsl(var(--border))",
                    borderRadius: "6px",
                  }}
                  labelFormatter={(label) => formatTimestamp(String(label))}
                  formatter={(value, name) => [
                    `${Number(value).toFixed(2)} GiB`,
                    name === "used" ? "Used" : "Total",
                  ]}
                />
                <Area
                  type="monotone"
                  dataKey="total"
                  stroke="#94a3b8"
                  strokeDasharray="4 4"
                  strokeWidth={1}
                  fillOpacity={0}
                  isAnimationActive={false}
                />
                <Area
                  type="monotone"
                  dataKey="used"
                  stroke="#10b981"
                  fillOpacity={1}
                  fill="url(#ramGradient)"
                  strokeWidth={2}
                  isAnimationActive={false}
                />
              </AreaChart>
            </ResponsiveContainer>
          )}
        </CardContent>
      </Card>
    </motion.div>
  );
}
