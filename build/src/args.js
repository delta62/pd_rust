import minimist from 'minimist'

let help = () => {
  console.log(`Usage: ${process.argv[1]} <example> [opts]
Compiles, runs, and debugs example Playdate apps written in Rust

Options
--debug    Debug the app using gdb once the build completes
--help     Show this help and exit
--release  Produce an optimized release built of the app
--run      Run the app in the Playdate simulator once the build completes
--sdkPath  Path to the root of the Playdate SDK

The SDK path specifies where the pdc and PlaydateSimulator commands are located.
There are three ways to specify the location, each taking precedence over the
next:
 - passing the --sdkPath argument
 - setting the SDK_PATH environment variable
 - default location of ~/.local/share/playdate-sdk`)
}

export let getArgs = () => {
  let argv = minimist(process.argv.slice(2))

  if (argv.help) {
    help()
    process.exit(1)
  }

  if (!argv._[0]) {
    console.error(
      'Please provide the name of an example app in the examples/ directory'
    )
    help()
    process.exit(1)
  }

  return argv
}
