import { invoke } from './tauri-wrapper';
import type { ParserConfig } from './types';

export type { ParserConfig };

export interface Site {
  id: number;
  name: string;
  url: string;
  parser_config: ParserConfig;
  created_at: string;
  updated_at: string;
}

export interface Mod {
  id: number;
  site_id: number;
  title: string;
  url: string;
  version?: string;
  author?: string;
  description?: string;
  image_url?: string;
  changes?: string;
  created_at: string;
  updated_at: string;
}

export interface Notification {
  id: number;
  mod_id: number;
  site_id: number;
  title: string;
  message: string;
  read: boolean;
  created_at: string;
}

export const api = {
  async getSites(): Promise<Site[]> {
    return invoke('get_sites');
  },

  async addSite(name: string, url: string, parserConfig: ParserConfig): Promise<Site> {
    return invoke('add_site', { name, url, parserConfig });
  },

  async updateSite(
    id: number,
    name: string,
    url: string,
    parserConfig: ParserConfig
  ): Promise<void> {
    return invoke('update_site', { id, name, url, parserConfig });
  },

  async deleteSite(id: number): Promise<void> {
    return invoke('delete_site', { id });
  },

  async getMods(siteId?: number): Promise<Mod[]> {
    return invoke('get_mods', { siteId });
  },

  async checkUpdates(siteId?: number): Promise<Mod[]> {
    return invoke('check_updates', { siteId });
  },

  async buildParser(html: string, selector: string): Promise<ParserConfig> {
    return invoke('build_parser', { html, selector });
  },

  async testParser(siteId: number): Promise<Mod[]> {
    return invoke('test_parser', { siteId });
  },

  async getNotifications(): Promise<Notification[]> {
    return invoke('get_notifications');
  },

  async markNotificationRead(id: number): Promise<void> {
    return invoke('mark_notification_read', { id });
  },
};
