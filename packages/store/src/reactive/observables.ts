import { ESMap, Iterator, Set } from 'typescript';
import { Event } from '@muds/event';
import { Arena, Deque, Generator, List } from '../collection';
import { CollectionEvents } from './collection';

/** An observable arena. */
export class ObservableArena<T, I extends number = number>
  implements Arena<T, I>, CollectionEvents<I, T>
{
  public readonly onClear: Event<[thisArg: this]> = Event.create();
  public readonly onAdd: Event<[thisArg: this, key: I, value: T]> = Event.create();
  public readonly onDelete: Event<[thisArg: this, key: I, value: T]> = Event.create();
  public readonly onUpdate: Event<[thisArg: this, index: I, value: T, prevValue: T]> = Event.create();

  public constructor(
    /** The underlying arena. */
    protected readonly arena: Arena<T, I>
  ) {
  }

  public get size(): number {
    return this.arena.size;
  }

  public has(key: I): boolean {
    return this.arena.has(key);
  }

  public get(key: I): T | undefined {
    return this.arena.get(key);
  }

  public add(value: T): I {
    const key = this.arena.add(value);
    this.onAdd.emit(this, key, value);
    return key;
  }

  public clear(): void {
    this.arena.clear();
    this.onClear.emit(this);
  }

  public delete(key: I): boolean {
    const value = this.arena.get(key);
    if (value !== undefined && this.arena.delete(key)) {
      this.onDelete.emit(this, key, value);
      return true;
    }
    return false;
  }

  public entries(): Iterator<[I, T]> {
    return this.arena.entries();
  }

  public keys(): Iterator<I> {
    return this.arena.keys();
  }

  public set(key: I, value: T): this {
    const prevValue = this.arena.get(key);
    if (prevValue !== undefined) {
      this.arena.set(key, value);
      this.onUpdate.emit(this, key, value, prevValue);
    }
    return this;
  }

  public values(): Iterator<T> {
    return this.arena.values();
  }

  public forEach(action: (value: T, key: I) => void): void {
    this.arena.forEach(action);
  }

  public [Symbol.iterator](): Iterator<[I, T]> {
    return this.arena.entries();
  }
}

/** An observable deque. */
export class ObservableDeque<T> implements Deque<T>, CollectionEvents<number, T> {
  public readonly onClear: Event<[thisArg: this]> = Event.create();
  public readonly onAdd: Event<[thisArg: this, index: number, value: T]> = Event.create();
  public readonly onDelete: Event<[thisArg: this, index: number, value: T]> = Event.create();

  public constructor(
    /** The underlying deque. */
    protected readonly deque: Deque<T>
  ) {
  }

  public get size(): number {
    return this.deque.size;
  }

  public get first(): T | undefined {
    return this.deque.first;
  }

  public get last(): T | undefined {
    return this.deque.last;
  }

  public clear(): void {
    this.deque.clear();
    this.onClear.emit(this);
  }

  public unshift(value: T): void {
    this.deque.unshift(value);
    this.onAdd.emit(this, 0, value);
  }

  public shift(): T | undefined {
    const value = this.deque.shift();
    (value !== undefined) && this.onDelete.emit(this, 0, value);
    return value;
  }

  public push(value: T): void {
    this.deque.push(value);
    this.onAdd.emit(this, this.deque.size - 1, value);
  }

  public pop(): T | undefined {
    const value = this.deque.pop();
    (value !== undefined) && this.onDelete.emit(this, this.deque.size, value);
    return value;
  }
}

/** An observable generator. */
export class ObservableGenerator<T> implements Generator<T>, CollectionEvents<T, T> {
  public readonly onClear: Event<[thisArg: this]> = Event.create();
  public readonly onAdd: Event<[thisArg: this, key: T, value: T]> = Event.create();
  public readonly onDelete: Event<[thisArg: this, key: T, value: T]> = Event.create();

  public constructor(
    /** The underlying generator. */
    protected readonly generator: Generator<T>
  ) {
  }

  public get size(): number {
    return this.generator.size;
  }

  public has(key: T): boolean {
    return this.generator.has(key);
  }

  public add(): T {
    const key = this.generator.add();
    this.onAdd.emit(this, key, key);
    return key;
  }

  public clear(): void {
    this.generator.clear();
    this.onClear.emit(this);
  }

  public delete(key: T): boolean {
    if (this.generator.delete(key)) {
      this.onDelete.emit(this, key, key);
      return true;
    }
    return false;
  }

  public keys(): Iterator<T> {
    return this.generator.keys();
  }

  public values(): Iterator<T> {
    return this.generator.values();
  }

  public entries(): Iterator<[T, T]> {
    return this.generator.entries();
  }

  public forEach(action: (value: T, key: T) => void): void {
    this.generator.forEach(action);
  }

  public [Symbol.iterator](): Iterator<T> {
    return this.generator.values();
  }
}

/** An observable list. */
export class ObservableList<T> implements List<T>, CollectionEvents<number, T> {
  public readonly onClear: Event<[thisArg: this]> = Event.create();
  public readonly onAdd: Event<[thisArg: this, index: number, value: T]> = Event.create();
  public readonly onDelete: Event<[thisArg: this, index: number, value: T]> = Event.create();
  public readonly onUpdate: Event<[thisArg: this, index: number, value: T, prevValue: T]> = Event.create();

  public constructor(
    /** The underlying list. */
    protected readonly list: List<T>
  ) {
  }

  public get size(): number {
    return this.list.size;
  }

  public get last(): T | undefined {
    return this.list.last;
  }

  public get(index: number): T | undefined {
    return this.list.get(index);
  }

  public has(index: number): boolean {
    return this.list.has(index);
  }

  public clear(): void {
    this.list.clear();
    this.onClear.emit(this);
  }

  public pop(): T | undefined {
    const value = this.list.pop();
    if (value !== undefined) {
      this.onDelete.emit(this, this.list.size, value);
    }
    return value;
  }

  public push(value: T): void {
    this.list.push(value);
    this.onAdd.emit(this, this.list.size - 1, value);
  }

  public set(index: number, value: T): this {
    const oldValue = this.list.get(index);
    this.list.set(index, value);
    if (oldValue === undefined) {
      this.onAdd.emit(this, index, value);
    } else {
      this.onUpdate.emit(this, index, value, oldValue);
    }
    return this;
  }

  public entries(): Iterator<[number, T]> {
    return this.list.entries();
  }

  public keys(): Iterator<number> {
    return this.list.keys();
  }

  public values(): Iterator<T> {
    return this.list.values();
  }

  public forEach(action: (value: T, index: number) => void): void {
    return this.list.forEach(action);
  }

  public [Symbol.iterator](): Iterator<T> {
    return this.list.values();
  }
}

/** An observable map. */
export class ObservableMap<K, V> implements ESMap<K, V>, Iterable<[K, V]>, CollectionEvents<K, V> {
  public readonly onClear: Event<[thisArg: this]> = Event.create();
  public readonly onAdd: Event<[thisArg: this, key: K, value: V]> = Event.create();
  public readonly onDelete: Event<[thisArg: this, key: K, value: V]> = Event.create();
  public readonly onUpdate: Event<[thisArg: this, key: K, value: V, prevValue: V]> = Event.create();

  public constructor(
    /** The underlying map. */
    protected readonly map: ESMap<K, V>
  ) {
  }

  public get size(): number {
    return this.map.size;
  }

  public get(key: K): V | undefined {
    return this.map.get(key);
  }

  public has(key: K): boolean {
    return this.map.has(key);
  }

  public clear(): void {
    this.map.clear();
    this.onClear.emit(this);
  }

  public delete(key: K): boolean {
    const oldValue = this.map.get(key);
    if (oldValue !== undefined && this.map.delete(key)) {
      this.onDelete.emit(this, key, oldValue);
      return true;
    }
    return false;
  }

  public set(key: K, value: V): this {
    const oldValue = this.map.get(key);
    this.map.set(key, value);
    if (oldValue === undefined) {
      this.onAdd.emit(this, key, value);
    } else {
      this.onUpdate.emit(this, key, value, oldValue);
    }
    return this;
  }

  public entries(): Iterator<[K, V]> {
    return this.map.entries();
  }

  public keys(): Iterator<K> {
    return this.map.keys();
  }

  public values(): Iterator<V> {
    return this.map.values();
  }

  public forEach(action: (value: V, key: K) => void): void {
    this.map.forEach(action);
  }

  public [Symbol.iterator](): Iterator<[K, V]> {
    return this.map.entries();
  }
}

/** An observable set. */
export class ObservableSet<T> implements Set<T>, Iterable<T>, CollectionEvents<T, T> {
  public readonly onClear: Event<[thisArg: this]> = Event.create();
  public readonly onAdd: Event<[thisArg: this, key: T, value: T]> = Event.create();
  public readonly onDelete: Event<[thisArg: this, key: T, value: T]> = Event.create();

  public constructor(
    /** The underlying set. */
    protected readonly set: Set<T>
  ) {
  }

  public get size(): number {
    return this.set.size;
  }

  public has(value: T): boolean {
    return this.set.has(value);
  }

  public clear(): void {
    this.set.clear();
    this.onClear.emit(this);
  }

  public delete(value: T): boolean {
    if (this.set.delete(value)) {
      this.onDelete.emit(this, value, value);
      return true;
    }
    return false;
  }

  public add(value: T): this {
    this.set.add(value);
    this.onAdd.emit(this, value, value);
    return this;
  }

  public entries(): Iterator<[T, T]> {
    return this.set.entries();
  }

  public keys(): Iterator<T> {
    return this.set.keys();
  }

  public values(): Iterator<T> {
    return this.set.values();
  }

  public forEach(action: (value: T, key: T) => void): void {
    this.set.forEach(action);
  }

  public [Symbol.iterator](): Iterator<T> {
    return this.set.values();
  }
}
