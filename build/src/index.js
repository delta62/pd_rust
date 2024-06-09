#!/usr/bin/env node

import { copyFile, mkdir, stat, unlink } from 'fs/promises'
import * as path from 'path'
import { homedir } from 'os'
import { spawnWithLogs, localPath, nop } from './util.js'
import { reportStep, reportBuildComplete } from './output.js'
import { getArgs } from './args.js'

let startTime = Date.now()
let argv = getArgs()

const CLEAN_FILES = ['pdex.so']
const SOURCE_DIR = 'Source'
const SDK_PATH =
  argv.sdkPath ??
  process.env['SDK_PATH'] ??
  path.join(homedir(), '.local', 'share', 'playdate-sdk')

let validateExample = async example => {
  let examplePath = localPath('examples', example, 'Cargo.toml')
  let result = await stat(examplePath)
  if (!result.isFile()) {
    throw new Error("Cargo.toml exists, but it isn't a file")
  }
}

let ensureOutputDir = async example => {
  let outDir = localPath('examples', example, SOURCE_DIR)
  await mkdir(outDir, { recursive: true }).catch(nop)
}

let clean = async () => {
  reportStep('Cleaning build directory')
  let promises = CLEAN_FILES.map(file => {
    let p = localPath(SOURCE_DIR, file)

    return unlink(p)
      .then(() => reportSubstep(`Removed ${p}`))
      .catch(nop)
  })

  await Promise.all(promises)
}

let build = async example => {
  let cwd = localPath()
  let buildArgs = ['build']
  if (argv.release) buildArgs.push('--release')
  await spawnWithLogs('Building API', 'cargo', buildArgs, { cwd })

  cwd = localPath('examples', example)
  await spawnWithLogs('Building Game', 'cargo', buildArgs, { cwd })

  reportStep('Copying lib to Playdate Binary')
  let src = localPath(
    'examples',
    example,
    'target',
    'debug',
    `lib${example}.so`
  )
  let dest = localPath('examples', example, SOURCE_DIR, 'pdex.so')
  await copyFile(src, dest)

  let pdc = path.join(SDK_PATH, 'bin', 'pdc')
  let sourceDir = localPath('examples', example, SOURCE_DIR)
  let args = ['-sdkpath', SDK_PATH, sourceDir, `${example}.pdx`]
  await spawnWithLogs('Running Playdate Compiler', pdc, args)
}

let run = async example => {
  let fileName = `${example}.pdx`
  let cmd = path.join(SDK_PATH, 'bin', 'PlaydateSimulator')
  let pdxPath = localPath(fileName)
  await spawnWithLogs(`Running ${fileName}`, cmd, [pdxPath], {
    stdio: 'inherit',
  })
}

let debug = async () => {
  let pdCommand = path.join(SDK_PATH, 'bin', 'PlaydateSimulator')
  let pdxPath = localPath(`${example}.pdx`)
  let args = ['--silent', '--args', pdCommand, pdxPath]
  await spawnWithLogs(`Starting GDB`, 'rust-gdb', args, { stdio: 'inherit' })
}

let example = argv._[0]

await validateExample(example)
await ensureOutputDir(example)
await clean()
await build(example)
reportBuildComplete({ dest: `${example}.pdx`, startTime, endTime: Date.now() })

if (argv.run) {
  await run(example)
} else if (argv.debug) {
  await debug(example)
}
