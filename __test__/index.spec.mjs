import test from 'ava'
import path from 'path'

import { getEtag } from '../index.js'

test('etag from native', async (t) => {
  const etag = await getEtag(path.join(process.cwd(), '/test.mp3'))
  t.is(etag, 'FoBG5ARO2FtfAvY0JJnnucTWEctV')
})
