import type {
  Node,
  CPUData,
  RAMData,
  RAMStatRaw,
  CPUStatRaw,
  Service,
  ServiceStatus,
  NodeInfo,
  LoginCredentials,
  LoginResponse,
  UserDetails,
  ChangePasswordCredentials,
} from "@/types";

// ─── Config ───────────────────────────────────────────────────────────────────
const getApiUrl = () => {
  if (typeof window !== 'undefined' && (window as any).__ENV__?.API_URL) {
    return (window as any).__ENV__.API_URL;
  }
  return process.env.API_URL || process.env.NEXT_PUBLIC_API_URL || '';
};

const getAuthUrl = () => {
  if (typeof window !== 'undefined' && (window as any).__ENV__?.AUTH_URL) {
    return (window as any).__ENV__.AUTH_URL;
  }
  return process.env.AUTH_URL || process.env.NEXT_PUBLIC_AUTH_URL || '';
};

const getHeaders = () => {
  const token =
    typeof window !== "undefined" ? localStorage.getItem("ferro_token") : "";
  return {
    Authorization: token || "",
    "Content-Type": "application/json",
  };
};

// ─── Generic Response Handler ─────────────────────────────────────────────────
const handleResponse = async <T>(
  response: Response,
  defaultValue: T | null = null,
): Promise<T | null> => {
  if (response.status === 401) {
    if (typeof window !== "undefined") {
      localStorage.removeItem("ferro_token");
      window.location.href = "/login";
    }
    return defaultValue;
  }

  if (response.status === 204) {
    return defaultValue;
  }

  if (!response.ok) {
    const errorData = await response.json().catch(() => ({}));
    throw new Error(errorData.message || `API error: ${response.status}`);
  }

  return response.json() as Promise<T>;
};

// ─── API ──────────────────────────────────────────────────────────────────────
export const api = {
  async getNodes(): Promise<Node[]> {
    const response = await fetch(`${getApiUrl()}/get_node_list`, {
      method: "POST",
      headers: getHeaders(),
    });
    return (await handleResponse<Node[]>(response, [])) ?? [];
  },

  async getLatestCPU(nodeId: number): Promise<CPUData> {
    const response = await fetch(`${getApiUrl()}/get_latest_cpu?node=${nodeId}`, {
      method: "POST",
      headers: getHeaders(),
    });

    const data = await handleResponse<CPUStatRaw>(response, {
      value: 0,
      date_time: new Date().toISOString(),
    });

    return {
      cpu: data?.value ?? 0,
      timestamp: data?.date_time ?? new Date().toISOString(),
    };
  },

  async getLatestRAM(nodeId: number): Promise<RAMData | null> {
    const response = await fetch(`${getApiUrl()}/get_latest_ram?node=${nodeId}`, {
      method: "POST",
      headers: getHeaders(),
    });
    return handleResponse<RAMData>(response);
  },

  async getCPUHistory(nodeId: number): Promise<CPUData[]> {
    const response = await fetch(`${getApiUrl()}/cpu_stat?node=${nodeId}`, {
      method: "POST",
      headers: getHeaders(),
    });

    const data = await handleResponse<CPUStatRaw[]>(response, []);

    return (data ?? []).map((item: CPUStatRaw) => ({
      cpu: item.value,
      timestamp: item.date_time,
    }));
  },

  async getRAMHistory(nodeId: number): Promise<RAMData[]> {
    const response = await fetch(`${getApiUrl()}/ram_stat?node=${nodeId}`, {
      method: "POST",
      headers: getHeaders(),
    });

    const data = await handleResponse<RAMStatRaw[]>(response, []);

    return (data ?? []).map((item: RAMStatRaw) => ({
      free: item.free,
      total: item.total,
      timestamp: item.timestamp,
    }));
  },

  async getNodeServices(nodeId: number): Promise<Service[]> {
    const response = await fetch(`${getApiUrl()}/node_services?node=${nodeId}`, {
      method: "POST",
      headers: getHeaders(),
    });
    return (await handleResponse<Service[]>(response, [])) ?? [];
  },

  async getServiceStatus(nodeId: number): Promise<ServiceStatus[]> {
    const response = await fetch(
      `${getApiUrl()}/service_current_stat?node=${nodeId}`,
      {
        method: "POST",
        headers: getHeaders(),
      },
    );
    return (await handleResponse<ServiceStatus[]>(response, [])) ?? [];
  },

  async userLogin(
    credentials: LoginCredentials,
  ): Promise<LoginResponse | null> {
    const response = await fetch(`${getAuthUrl()}/user_login`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        username: credentials.username,
        password: credentials.password || "",
      }),
    });
    return handleResponse<LoginResponse>(response);
  },

  async getNodeInfo(nodeId: number): Promise<NodeInfo | null> {
    const response = await fetch(`${getApiUrl()}/get_node_info?node=${nodeId}`, {
      method: "POST",
      headers: getHeaders(),
    });
    return handleResponse<NodeInfo>(response);
  },

  async getUserDetails(): Promise<UserDetails | null> {
    const response = await fetch(`${getApiUrl()}/get_userdetails`, {
      method: "POST",
      headers: getHeaders(),
    });
    return handleResponse<UserDetails>(response);
  },

  async changePassword(credentials: ChangePasswordCredentials): Promise<boolean> {
    const response = await fetch(`${getApiUrl()}/change_password`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify(credentials),
    });

    if (response.status === 200) return true;
    if (response.status === 409) return false;

    // Fallback for other errors handled by handleResponse logic or similar
    if (!response.ok) {
      const errorData = await response.json().catch(() => ({}));
      throw new Error(errorData.message || `API error: ${response.status}`);
    }
    return true;
  },

  getCPUStreamUrl(nodeId: number): string {
    const streamBase = getApiUrl()?.replace("/view", "/stream") || "";
    return `${streamBase}/cpu?node=${nodeId}`;
  },

  getRAMStreamUrl(nodeId: number): string {
    const streamBase = getApiUrl()?.replace("/view", "/stream") || "";
    return `${streamBase}/ram?node=${nodeId}`;
  },

  async createNode(name: string): Promise<{ token: string } | null> {
    const response = await fetch(`${getApiUrl()}/create_nodes`, {
      method: "POST",
      headers: getHeaders(),
      body: JSON.stringify({ name }),
    });
    return handleResponse<{ token: string }>(response);
  },
};
