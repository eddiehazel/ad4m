import {
    ApolloClient,
    InMemoryCache,
} from "@apollo/client/core";
import { WebSocketLink } from '@apollo/client/link/ws';
import ws from "ws"
import main from "../main";
import path from "path";
import { OuterConfig } from "../types";
import { Ad4mClient, LanguageMetaInput } from "@perspect3vism/ad4m";
import fs from "fs-extra";
import { exit } from "process";

const TEST_DIR = path.join(`${__dirname}/../../src/test-temp`);
const appDataPath = path.join(TEST_DIR, "agents", "publishing-agent");
const ipfsRepoPath = path.join(appDataPath);
const publishLanguagesPath = path.join(TEST_DIR, "languages");
const publishingAgentPath = path.join(`${__dirname}/../../src/tests/publishing-agent`);
const publishingBootstrapSeedPath = path.join(`${__dirname}/../../src/tests/publishBootstrapSeed.json`);
const bootstrapSeedPath = path.join(`${__dirname}/../../src/tests/publishBootstrapSeed.json`);

//Update this as new languages are needed within testing code
const languagesToPublish = {
    "agent-expression-store": {name: "agent-expression-store", description: "", possibleTemplateParams: ["id", "name", "description"]} as LanguageMetaInput, 
    "direct-message-language": {name: "direct-message-language", description: "", possibleTemplateParams: ["id", "name", "description"]} as LanguageMetaInput, 
    "neighbourhood-store": {name: "neighbourhood-store", description: "", possibleTemplateParams: ["id", "name", "description"]} as LanguageMetaInput, 
    "note-ipfs": {name: "note-ipfs", description: "", possibleTemplateParams: ["id", "name", "description"]} as LanguageMetaInput, 
    "social-context": {name: "social-context", description: "", possibleTemplateParams: ["id", "name", "description"]} as LanguageMetaInput
}

const languageHashes = {
    "directMessageLanguage": "",
    "agentLanguage": "",
    "perspectiveLanguage": "",
    "neighbourhoodLanguage": ""
}

function apolloClient(port: number): ApolloClient<any> {
    return new ApolloClient({
      link: new WebSocketLink({
          uri: `http://localhost:${port}/graphql`,
          options: { reconnect: true },
          webSocketImpl: ws,
      }),
      cache: new InMemoryCache({resultCaching: false, addTypename: false}),
      defaultOptions: {
          watchQuery: {
              fetchPolicy: "no-cache",
          },
          query: {
              fetchPolicy: "no-cache",
          }
      },
  });
}

function createTestingAgent() {
    if (!fs.existsSync(appDataPath)) {
        fs.copySync(publishingAgentPath, appDataPath);
    }
}

function injectSystemLanguages() {
    if (fs.existsSync(bootstrapSeedPath)) {
        const bootstrapSeed = JSON.parse(fs.readFileSync(bootstrapSeedPath).toString());
        bootstrapSeed["directMessageLanguage"] = languageHashes["directMessageLanguage"];
        bootstrapSeed["agentLanguage"] = languageHashes["agentLanguage"];
        bootstrapSeed["perspectiveLanguage"] = languageHashes["perspectiveLanguage"];
        bootstrapSeed["neighbourhoodLanguage"] = languageHashes["neighbourhoodLanguage"];
        fs.writeFileSync(bootstrapSeedPath, JSON.stringify(bootstrapSeed));
    } else {
        throw new Error(`Could not find boostrapSeed at path: ${bootstrapSeedPath}`)
    }
}

async function publish() {
    createTestingAgent();
    const core = await main.init({
        appDataPath,
        resourcePath: TEST_DIR,
        networkBootstrapSeed: publishingBootstrapSeedPath,
        languageLanguageOnly: true,
        bootstrapFixtures: {
          languages: [],
          perspectives: [],
        },
        mocks: false,
        ipfsRepoPath,
        hcUseBootstrap: false,
        hcUseProxy: false,
        hcUseLocalProxy: false,
        hcUseMdns: true
    } as OuterConfig)

    core.initControllers()
    await core.initLanguages()

    const ad4mClient = new Ad4mClient(apolloClient(4000));
    await ad4mClient.agent.unlock("passphrase");

    for (const [language, languageMeta] of Object.entries(languagesToPublish)) {
        let bundlePath = path.join(publishLanguagesPath, language, "build", "bundle.js");
        console.log("Attempting to publish language", bundlePath);
        let publishedLang = await ad4mClient.languages.publish(bundlePath, languageMeta);
        console.log("Published with result", publishedLang);
        if (language === "agent-expression-store") {
            languageHashes["agentLanguage"] = publishedLang.address;
        }
        if (language === "neighbourhood-store") {
            languageHashes["neighbourhoodLanguage"] = publishedLang.address;
        }
        if (language === "direct-message-language") {
            languageHashes["directMessageLanguage"] = publishedLang.address;
        }
    }
    injectSystemLanguages()
    await core!.exit();
    exit();
}

publish()