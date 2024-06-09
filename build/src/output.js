import chalk from 'chalk'

export let reportStep = description => {
  console.log(`${chalk.blue('=>')} ${description}`)
}

export let reportSpawn = (cmd, args) => {
  console.log(`${chalk.blue('=>')} Running ${cmd} ${args.join(' ')}`)
}

export let reportFailed = ({ cmd, args, err, stdout, stderr }) => {
  let message = `Error: ${cmd} ${args.join(' ')} failed`
  console.error(message)
  stdout && console.log(stdout.toString())
  stderr && console.error(stderr.toString())
  err && console.error(err)
}

export let reportBuildComplete = ({ dest, startTime, endTime }) => {
  let duration = `${endTime - startTime}ms`
  console.log(
    `${chalk.green('📦')} Built ${chalk.blue(dest)} in ${chalk.blue(duration)}`
  )
}
