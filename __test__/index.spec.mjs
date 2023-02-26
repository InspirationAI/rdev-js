import test from 'ava'
import {
  rdev,
  testLeftMouseButton,
  testMoveMouse,
  testPressSKey,
  testReleaseLeftMouseButton, testReleaseRightMouseButton,
  testReleaseSKey, testRightMouseButton, testWheel
} from "../index.js";

const getEvent = async (t) => {
  return new Promise((resolve) => {
    rdev((err, event) => {
      if (err) {
        t.fail(err)
      }
      resolve(event)
    })
  })
}

test.serial('Type S key', async (t) => {
  let event = getEvent(t)
  testPressSKey()
  event = await event
  t.is(event.type, 'keydown')
  t.is(event.value.key, 'KeyS')

  event = getEvent(t)
  testReleaseSKey()
  event = await event
  t.is(event.type, 'keyup')
  t.is(event.value.key, 'KeyS')
})

test.serial('Move mouse', async (t) => {
  let event = getEvent(t)
  testMoveMouse(100, 100)
  event = await event
  t.is(event.type, 'mousemove')
  t.is(event.value.x, 100)
  t.is(event.value.y, 100)
})

test.serial('Click left button of the mouse', async (t) => {
  let event = getEvent(t)
  testLeftMouseButton()
  event = await event
  t.is(event.type, 'mousedown')
  t.is(event.value.button, 'Left')

  event = getEvent(t)
  testReleaseLeftMouseButton()
  event = await event
  t.is(event.type, 'mouseup')
  t.is(event.value.button, 'Left')
})

test.serial('Click right button of the mouse', async (t) => {
  let event = getEvent(t)
  testRightMouseButton()
  event = await event
  t.is(event.type, 'mousedown')
  t.is(event.value.button, 'Right')

  event = getEvent(t)
  testReleaseRightMouseButton()
  event = await event
  t.is(event.type, 'mouseup')
  t.is(event.value.button, 'Right')
})

test.serial('Move the wheel', async (t) => {
  let event = getEvent(t)
  testWheel(100, 100)
  event = await event
  t.is(event.type, 'wheel')
  t.is(event.value.deltaX, 100)
  t.is(event.value.deltaY, 100)
})