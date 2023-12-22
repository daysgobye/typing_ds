import { Subscribable } from "./subscribable";

export class ProgramSettings extends Subscribable {
  private static instance: ProgramSettings;
  private _autoConnect: boolean = true;
  private _autoFileTree: boolean = false;
  private _debug: boolean = false;
  private _dev: boolean = false;
  private _apiUrl: string = "https://getpeg.xyz/api/";
  private _PPP: boolean = false;
  PppBuyLink: string = "https://boardsource.xyz/store/5f2efc462902de7151495057";

  public version: string = "v0.1";

  private saveState = () => {
    const isBrowser = typeof window !== "undefined";

    this.updateSubScribers();
    if (isBrowser) {
      localStorage.setItem(
        "cpy_toolbox_settings",
        JSON.stringify({
          autoConnect: this._autoConnect,
          debug: this._debug,
        })
      );
    }
  };
  private constructor() {
    super();
    const isBrowser = typeof window !== "undefined";
    if (isBrowser) {
      const pastStateStr = localStorage.getItem("cpy_toolbox_settings");

      const pastState = JSON.parse(pastStateStr ? pastStateStr : "{}");
      if (pastState) {
        this._debug = pastState.debug;
        this._autoConnect = pastState.autoConnect;
      }
    }
  }

  public get autoConnect() {
    return this._autoConnect;
  }
  public set autoConnect(newValue: boolean) {
    this._autoConnect = newValue;
    this.saveState();
  }
  public get autoFileTree() {
    return this._autoFileTree;
  }
  public set autoFileTree(newValue: boolean) {
    this._autoFileTree = newValue;
    this.saveState();
  }
  public get debug() {
    return this._debug;
  }
  public set debug(newValue: boolean) {
    this._debug = newValue;
    this.saveState();
  }

  public get dev() {
    return this._dev;
  }
  public set dev(newValue: boolean) {
    this._dev = newValue;
    this.saveState();
  }

  public get apiUrl() {
    return this._apiUrl;
  }
  public set PPP(newValue: boolean) {
    if (this._PPP !== newValue) {
      this._PPP = newValue;
      this.saveState();
    }
  }
  public get PPP() {
    return this._PPP;
  }
  public set apiUrl(newValue: string) {
    this._apiUrl = newValue;
    this.saveState();
  }

  public static getInstance(): ProgramSettings {
    if (!ProgramSettings.instance) {
      ProgramSettings.instance = new ProgramSettings();
    }
    return ProgramSettings.instance;
  }
}
