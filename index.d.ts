/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface RdevEventValue {
  x?: number
  y?: number
  deltaX?: number
  deltaY?: number
  key?: string
  button?: string
}
export interface RdevEvent {
  type: string
  code?: string
  value: RdevEventValue
  time: number
}
export function rdev(callback: (err: null | Error, event: RdevEvent) => void): void
export function testPressSKey(): void
export function testReleaseSKey(): void
export function testMoveMouse(x: number, y: number): void
export function testLeftMouseButton(): void
export function testReleaseLeftMouseButton(): void
export function testRightMouseButton(): void
export function testReleaseRightMouseButton(): void
export function testWheel(deltaX: number, deltaY: number): void
