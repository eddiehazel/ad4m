// //@ts-ignore
// import swipl from 'swipl-stdio'
import { MainConfig } from "./Config.ts";

export default class PrologInstance {
    #engine?: any

    constructor(config: MainConfig) {
        // this.#engine = new swipl.Engine(
        //     config.swiplPath ? config.swiplPath : path.join(config.resourcePath, "swipl"),
        //     config.swiplHomePath
        // )
    }

    async query(input: string) {
        const query = await this.#engine.createQuery(input);
        let allMatches = []
        try {
            let ret = null;
            while (ret = await query.next()) {
                allMatches.push(ret)
            }
        } finally {
            await query.close()
        }

        if(JSON.stringify(allMatches) === JSON.stringify([{}]))
            return true
        if(JSON.stringify(allMatches) === JSON.stringify([]))
            return false

        return allMatches
    }

    async call(query: string) {
        return await this.#engine.call(query)
    };

    async consult(program: string) {
        // const tmpobj = tmp.fileSync()
        // //@ts-ignore
        // fs.writeFileSync(tmpobj.name, program);
        // const result = await this.call(`consult('${tmpobj.name}').`)
        // tmpobj.removeCallback()
        return program
    }

    close() {
        this.#engine.close()
    }
}