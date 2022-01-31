import("./core/PerspectivismCore");  // Don't delete this line.
import { PerspectivismCore } from "./core/PerspectivismCore";

//Module types
export interface OuterConfig {
  resourcePath: string
  appDataPath: string
  appDefaultLangPath: string
  ad4mBootstrapLanguages: BootstrapLanguages,
  ad4mBootstrapFixtures?: BootstrapFixtures,
  appBuiltInLangs?: string[],
  appLangAliases?: object,
  mocks: boolean,
  gqlPort?: number,
  hcPortAdmin?: number,
  hcPortApp?: number,
  ipfsSwarmPort?: number,
  ipfsRepoPath?: string
  hcUseLocalProxy?: boolean,
  hcUseMdns?: boolean,
  hcUseProxy?: boolean,
  hcUseBootstrap?: boolean,
  connectHolochain?: boolean,
}

export interface Config {
  rootConfigPath: string
  dataPath: string
  languagesPath: string
  tempLangPath: string
  holochainPath: string
  holochainDataPath: string
  holochainConductorPath: string
  resourcePath: string
  builtInLangPath: string
  builtInLangs: string
  languageAliases: string
  bootstrapFixtures: BootstrapFixtures|void
}

export function init(config: OuterConfig): Promise<PerspectivismCore>;

export {PerspectivismCore};