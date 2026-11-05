// A minimal mounted PocketJS app for the Consortium Pocket HMI backend.
//
// `mount()` installs the frame contract that `consortium-hmi-pocket` drives.
// A manifest selects this file with `[hmi] app = "status-main"`.
import { mount } from '@pocketjs/framework/solid'
import { Text, View } from '@pocketjs/framework/solid/components'

function Status() {
  return (
    <View class="h-full w-full flex-col items-center justify-center gap-2 bg-slate-900">
      <Text class="text-2xl font-bold text-white">Consortium</Text>
      <Text class="text-sm text-slate-400">Pocket HMI running</Text>
    </View>
  )
}

mount(() => <Status />)
