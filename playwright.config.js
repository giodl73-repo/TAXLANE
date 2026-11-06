import {defineConfig} from '@playwright/test';
const port=Number(process.env.TAXLANE_TEST_PORT||8766);
if(!Number.isInteger(port)||port<1024||port>65535)throw Error('Invalid test port');
const origin=process.env.TAXLANE_SITE_URL||`http://127.0.0.1:${port}`;
const url=new URL(origin);
if(!['http:','https:'].includes(url.protocol)||url.username||url.password||url.pathname!=='/')throw Error('Use an HTTP(S) origin without credentials/path');
export default defineConfig({
  testDir:'./tests/browser',timeout:60000,
  use:{baseURL:origin,browserName:'chromium',launchOptions:process.env.TAXLANE_BROWSER_PATH?{executablePath:process.env.TAXLANE_BROWSER_PATH}:{}},
  webServer:process.env.TAXLANE_SITE_URL?undefined:{command:'python tools/serve-pages.py',url:`http://127.0.0.1:${port}/TAXLANE/`,reuseExistingServer:false},
});
