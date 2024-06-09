import * as childProcess from 'child_process'
import * as path from 'path'
import { reportSpawn, reportFailed, reportStep } from './output.js'

export let localPath = (...parts) => {
  return path.join(import.meta.dirname, '..', '..', ...parts)
}

let spawn = async (command, args, opts) => {
  let child = childProcess.spawn(command, args, opts)
  let stderr = null
  let stdout = null

  return new Promise((resolve, reject) => {
    child.once('error', err => {
      console.log('aasdfk', { err, stdout, stderr })
      reject({ err, stdout, stderr })
    })

    child.stdout?.on('data', data => {
      if (!stdout) {
        stdout = data
      } else {
        stdout = Buffer.concat(stdout, data)
      }
    })

    child.stderr?.on('data', data => {
      if (!stderr) {
        stderr = data
      } else {
        stderr = Buffer.concat([stderr, data])
      }
    })

    child.once('exit', code => {
      if (code === null || code !== 0) {
        let err = new Error(
          `${command} ${args.join(' ')} exited with code ${code}`
        )
        console.log('aasdfk', { err, stdout, stderr })
        reject({ err, stdout, stderr })
      } else {
        resolve({ stdout, stderr })
      }
    })
  })
}

export let spawnWithLogs = async (description, cmd, args, opts) => {
  reportStep(description)

  try {
    await spawn(cmd, args, opts)
  } catch (err) {
    reportFailed({ cmd, args, ...err })
    process.exit(1)
  }
}

export let nop = () => {}
