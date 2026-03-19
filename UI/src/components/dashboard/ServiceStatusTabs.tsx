"use client";

import { ServiceStatusGrouped, ServiceStatus } from "@/types";
import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { CheckCircle2, XCircle, ShieldAlert } from "lucide-react";

interface ServiceStatusTabsProps {
  services: ServiceStatusGrouped;
}

export function ServiceStatusTabs({ services }: ServiceStatusTabsProps) {
  const categories = Object.keys(services);

  if (categories.length === 0) {
    return (
      <div className="text-center py-8 text-muted-foreground italic border rounded-lg border-dashed">
        No services found for this node.
      </div>
    );
  }

  return (
    <Tabs defaultValue={categories[0]} className="w-full">
      <TabsList className="flex flex-wrap h-auto p-1 bg-muted/50 gap-1 mb-6">
        {categories.map((category) => (
          <TabsTrigger
            key={category}
            value={category}
            className="px-4 py-2 capitalize data-[state=active]:bg-background data-[state=active]:shadow-sm transition-all"
          >
            {category}
          </TabsTrigger>
        ))}
      </TabsList>

      {categories.map((category) => (
        <TabsContent
          key={category}
          value={category}
          className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 animate-in fade-in slide-in-from-bottom-2 duration-300"
        >
          {services[category].map((service) => (
            <ServiceCard key={service.service_name} service={service} />
          ))}
        </TabsContent>
      ))}
    </Tabs>
  );
}

function ServiceCard({ service }: { service: ServiceStatus }) {
  const isUp = service.status === "up";

  // SSL Expiry formatting: [year, dayOfYear, hour, minute, second, ...]
  const formatSSL = (ssl?: number[] | null) => {
    if (!ssl || ssl.length < 2) return null;
    const [year, dayOfYear] = ssl;
    const expiryDate = new Date(year, 0);
    expiryDate.setDate(dayOfYear);
    
    const now = new Date();
    const diffTime = expiryDate.getTime() - now.getTime();
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));
    
    let daysLeftText = '';
    let urgency: 'normal' | 'warn' | 'critical' = 'normal';

    if (diffDays > 0) {
      daysLeftText = `(${diffDays} days left)`;
      if (diffDays < 7) urgency = 'critical';
      else if (diffDays < 30) urgency = 'warn';
    } else if (diffDays === 0) {
      daysLeftText = `(Expires today)`;
      urgency = 'critical';
    } else {
      daysLeftText = `(Expired ${Math.abs(diffDays)} days ago)`;
      urgency = 'critical';
    }

    return {
      date: expiryDate.toLocaleDateString("en-US", {
        year: "numeric",
        month: "short",
        day: "numeric",
      }),
      daysLeft: diffDays,
      text: daysLeftText,
      urgency
    };
  };

  const sslInfo = formatSSL(service.ssl_exp);

  return (
    <div className="group relative flex flex-col gap-3 p-5 rounded-2xl border bg-card hover:shadow-md transition-all duration-300">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <div className={`p-2 rounded-full ${isUp ? 'bg-green-500/10 text-green-500' : 'bg-red-500/10 text-red-500'}`}>
            {isUp ? (
              <CheckCircle2 className="w-5 h-5" />
            ) : (
              <XCircle className="w-5 h-5" />
            )}
          </div>
          <div className="flex flex-col">
            <span className="font-bold text-base truncate max-w-[160px]" title={service.service_name}>
              {service.service_name}
            </span>
          </div>
        </div>
        <Badge
          variant={isUp ? "default" : "destructive"}
          className={`text-[10px] uppercase font-black tracking-widest px-2.5 py-0.5 rounded-full ${isUp ? 'bg-green-500/10 text-green-500 border-green-500/20 hover:bg-green-500/20' : ''}`}
        >
          {service.status}
        </Badge>
      </div>

      {service.error_msg && (
        <div className="text-xs text-red-500/90 font-medium bg-red-500/5 backdrop-blur-sm border border-red-500/10 p-3 rounded-xl leading-relaxed">
          {service.error_msg}
        </div>
      )}

      {sslInfo && (
        <div className={`flex flex-col gap-1 mt-auto pt-3 border-t border-muted/30`}>
          <div className="flex items-center gap-2">
            <ShieldAlert className={`w-4 h-4 ${
              sslInfo.urgency === 'critical' ? 'text-red-500' : 
              sslInfo.urgency === 'warn' ? 'text-amber-500' : 
              'text-blue-500'
            }`} />
            <span className="text-sm font-semibold tracking-tight">
              SSL Expires: {sslInfo.date}
            </span>
          </div>
          <span className={`text-xs ml-6 font-bold ${
            sslInfo.urgency === 'critical' ? 'text-red-500 animate-pulse' : 
            sslInfo.urgency === 'warn' ? 'text-amber-500' : 
            'text-muted-foreground'
          }`}>
            {sslInfo.text}
          </span>
        </div>
      )}
    </div>
  );
}
