interface AppConfiguration {
    name: string
    items: number
    active: boolean
}

function updateApp(config: Partial<AppConfiguration>) {
    //null-coalesing operator
    config.name = config.name ?? ""
    config.items = config.items ?? 0
    config.active = config.active ?? false

    //current solution
    config.name = typeof config.name === "string" ? config.name : "(no name)"
    config.items = typeof config.items === "number" ? config.items : -1
    config.active = typeof config.active === "boolean" ? config.active : true
}