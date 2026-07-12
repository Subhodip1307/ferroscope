"use client";

import { useEffect, useState } from "react";
import { motion } from "framer-motion";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
    LineChart,
    Line,
    XAxis,
    YAxis,
    CartesianGrid,
    Tooltip,
    ResponsiveContainer,
    Legend,
} from "recharts";
import { api } from "@/lib/api";
import { formatTimestamp } from "@/lib/utils";
import type { DiskData } from "@/types";

interface DiskChartProps {
    nodeId: number;
    nodeName: string;
}

export function DiskChart({ nodeId, nodeName }: DiskChartProps) {
    const [data, setData] = useState<DiskData[]>([]);
    const [loading, setLoading] = useState(false);
    const [isOffline, setIsOffline] = useState(false);

    //   const fetchHistory = useCallback(async () => {
    //     try {
    //       const history = await api.getDiskHistory(nodeId);

    //       setData(history.slice(0, 20));
    //       setIsOffline(false);
    //     } catch (error: any) {
    //       console.error("Error fetching disk history:", error);

    //       if (error.message?.includes("503")) {
    //         setIsOffline(true);
    //       }
    //     } finally {
    //       setLoading(false);
    //     }
    //   }, [nodeId]);

    useEffect(() => {
        // fetchHistory();

        const eventSource = new EventSource(api.getDiskStreamUrl(nodeId));

        eventSource.onmessage = (event) => {
            try {
                setIsOffline(false);

                const disks: DiskData[] = JSON.parse(event.data);

                // Combine all disks into one point
                const point: DiskData = {
                    read: disks.reduce((sum, disk) => sum + disk.read, 0),
                    write: disks.reduce((sum, disk) => sum + disk.write, 0),
                    timestamp:
                        disks[0]?.timestamp ?? new Date().toISOString(),
                };

                setData((prev) => {
                    const updated = [point, ...prev];
                    return updated.slice(0, 20);
                });
            } catch (err) {
                console.error("Error parsing Disk stream:", err);
            }
        };

        eventSource.onerror = () => {
            setIsOffline(true);
        };

        return () => eventSource.close();
    }, [nodeId]);

    const chartData = [...data]
        .map((item) => ({
            timestamp: item.timestamp,
            read: item.read,
            write: item.write,
        }))
        .reverse();

    const maxValue =
        chartData.length > 0
            ? Math.max(
                ...chartData.flatMap((d) => [d.read, d.write]),
                1
            )
            : 1;

    return (
        <motion.div
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ duration: 0.5 }}
        >
            <Card>
                <CardHeader>
                    <CardTitle className="text-lg">
                        Disk I/O (MiB/s) - {nodeName}
                    </CardTitle>
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
                            <div className="text-destructive font-semibold">
                                Node Offline
                            </div>

                            <div className="text-muted-foreground text-sm text-center px-4">
                                Wait for the node to come back online for real-time data.
                            </div>
                        </div>
                    ) : (
                        <ResponsiveContainer width="100%" height={300}>
                            <LineChart data={chartData}>
                                <CartesianGrid
                                    strokeDasharray="3 3"
                                    className="stroke-muted"
                                />

                                <XAxis
                                    dataKey="timestamp"
                                    tickFormatter={formatTimestamp}
                                    minTickGap={30}
                                    stroke="currentColor"
                                />

                                <YAxis
                                    domain={[0, Math.ceil(maxValue)]}
                                    stroke="currentColor"
                                    allowDecimals={false}
                                    tickFormatter={(value) => `${Math.round(Number(value))}`}
                                />

                                <Tooltip
                                    labelFormatter={(label) =>
                                        formatTimestamp(String(label))
                                    }
                                    formatter={(value, name) => [
                                        `${Number(value ?? 0).toFixed(2)} MB/s`,
                                        name === "read" ? "Read" : "Write",
                                    ]}
                                    contentStyle={{
                                        backgroundColor: "hsl(var(--background))",
                                        border: "1px solid hsl(var(--border))",
                                        borderRadius: "6px",
                                    }}
                                />

                                <Legend />

                                <Line
                                    type="monotone"
                                    dataKey="read"
                                    stroke="#3b82f6"
                                    strokeWidth={2}
                                    dot={false}
                                    isAnimationActive={false}
                                />

                                <Line
                                    type="monotone"
                                    dataKey="write"
                                    stroke="#f97316"
                                    strokeWidth={2}
                                    dot={false}
                                    isAnimationActive={false}
                                />
                            </LineChart>
                        </ResponsiveContainer>
                    )}
                </CardContent>
            </Card>
        </motion.div>
    );
}