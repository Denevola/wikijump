import { readable, Subscriber } from "svelte/store"
import { Api, RequestParams } from "../vendor/api"

const API_PATH = "/api--v0"

class WikijumpAPIInstance extends Api<void> {
  // private properties have _ as a prefix to prevent conflicting with any
  // autogenerated API methods

  /** Current CSRF token. */
  private declare _CSRF?: string

  /** The current base URL. If null, `API_PATH` will be used. */
  private _baseURL: string | null = null

  /** @param headers - Extra headers to send with every request. */
  constructor(headers: Record<string, string> = {}) {
    super({
      baseUrl: API_PATH,
      baseApiParams: {
        headers: {
          "Accept": "application/json",
          "Content-Type": "application/json",
          ...headers
        },
        secure: true,
        format: "json"
      },
      // this gets ran on every request,
      // so this is more for setting up an API request
      // than just handling security
      securityWorker: () => {
        const csrf = this._CSRF ?? getCSRFMeta()
        const xsrf = getCSRFCookie()
        const securityHeaders = xsrf
          ? { "X-CSRF-TOKEN": csrf, "X-XSRF-TOKEN": xsrf }
          : { "X-CSRF-TOKEN": csrf }
        return {
          baseUrl: this._baseURL ?? API_PATH,
          headers: securityHeaders
        } as RequestParams
      }
    })

    this._hijackAuthMethods()

    // update authentication status, as we may already be logged in
    this.authCheck().catch()
  }

  private _hijackAuthMethods() {
    // authLogin and authRefresh are special in that they regenerate your session.
    // this invalidates your old CSRF token, so we need to update it,
    // which means overriding the old methods with new ones.

    // additionally, we want to update the authed store to whatever
    // our authentication status is.
    // so, we need to hijack all the auth methods

    // unfortunately we can't use super.function because
    // the auto-generated "method" is actually a value and not a method.

    const login = this.authLogin.bind(this)
    const logout = this.authLogout.bind(this)
    const refresh = this.authRefresh.bind(this)
    const check = this.authCheck.bind(this)

    // we don't actually need to hijack authConfirm
    // const confirm = this.authConfirm.bind(this)

    this.authLogin = async (data, requestParams) => {
      const res = await login(data, requestParams)
      this._CSRF = res.csrf
      authSet(true)
      return res
    }

    this.authLogout = async requestParams => {
      await logout(requestParams)
      authSet(false)
    }

    this.authRefresh = async requestParams => {
      const res = await refresh(requestParams)
      this._CSRF = res.csrf
      return res
    }

    this.authCheck = async requestParams => {
      const res = await check(requestParams)
      authSet(res.authed)
      return res
    }
  }

  /**
   * Returns a base URL but for a different subdomain.
   *
   * @param subdomain - The subdomain to use.
   */
  subdomainURL(subdomain: string) {
    return `${window.location.protocol}//${subdomain}.${window.location.host}/${API_PATH}`
  }

  /**
   * Fires a callback in a context where the API's subdomain has been
   * changed to the one specified. Be aware of potential race conditions
   * when using this. If a race condition is unavoidable, you can use
   * {@link subdomainURL} and manually set the `baseURL` for the request.
   *
   * @param subdomain - The subdomain to use.
   * @param callback - The callback to run.
   */
  async withSubdomain(subdomain: string, callback: () => Promise<void>) {
    this._baseURL = this.subdomainURL(subdomain)
    await callback()
    this._baseURL = null
  }
}

let authSet: Subscriber<boolean>

/** Readable store holding the current authentication state. */
export const authed = readable(false, set => void (authSet = set))

let isAuthedBinding = false
authed.subscribe(state => void (isAuthedBinding = state))

/** Returns the current authentication state. */
export function isAuthenticated() {
  return isAuthedBinding
}

/** Wikijump API. */
export const WikijumpAPI = new WikijumpAPIInstance()

/**
 * Retrieves the CSRF token from the `<meta name="csrf-token" ...>` tag in
 * the `<head>`. This should always be present, so this function throws if
 * that element can't be found.
 */
function getCSRFMeta() {
  const meta = document.head.querySelector("meta[name=csrf-token]")
  if (!meta) throw new Error("No CSRF meta tag found")
  return meta.getAttribute("content")!
}

/** Retrieves the CSRF token from the `XSRF-TOKEN` cookie, if it exists. */
function getCSRFCookie() {
  const value = document.cookie
    .split(/;\s*/)
    .find(c => c.startsWith("XSRF-TOKEN="))
    ?.split("=")[1]
  return value
}
