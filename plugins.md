# Cynthia plug-ins
`Cyninst`, Cynthia's installer can also install plug-ins. To do this you can use:

```bash
cyninst -p [plugin_id] [plugin_version]
```
If you want that version to be '`latest`', you can omit the version parameter.

## List of available plugins

### HL-IMG for Cynthia

| Plug-in ID                    |                                                      `hlimg` |
| :---------------------------- | -----------------------------------------------------------: |
| Author                        |                                                 MLC Bloeiman |
| Source code                   | GitHub: <https://github.com/strawmelonjuice/hlimg-cynthiacms/>; NPM: <https://www.npmjs.com/package/hl-img> |
| License                       |                                                          MIT |
| Cynthia Plugin Loader version |                                                          `1` |

#### Installation: 

##### With Cyninst:

```bash
cyninst -p hlimg 1
```

#### About

Allows you to use [`hl-img`](https://www.npmjs.com/package/hl-img?activeTab=readme) in Cynthia-hosted pages and posts globally, without the need to edit template files or entering HTML.