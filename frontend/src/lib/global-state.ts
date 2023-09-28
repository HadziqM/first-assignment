//this to define global state on all process

import { writable } from "svelte/store"


//make typescript type
export interface Produk {
  id:number
  nama:string
  kategori:string
  harga:number
}

//get the host name
const domain = "http://127.0.0.1:8000"
//let make dummy data for testing
export const dummy:Produk = {
  id:0,
  nama:"dummy",
  kategori:"dummy",
  harga:100000
}

export const placeholder:Produk[] = [dummy,dummy,dummy]

//make global state data
export let gState = writable(placeholder)


//fuction to refresh list
export async function refresh() {
  //get request
  let res = await fetch(`${domain}/api/list`)
  if (res.status != 200) return false
  //update the global state on succesfull refresh
  gState.set(await res.json() as Produk[])
  return true
}


export async function dell(id:number) {
  //get request
  let res = await fetch(`${domain}/api/delete/${id}`)
  if (res.status != 200) return false
  return true
}

export async function add(nama:string,harga:number,kategori:string) {
  // post request json
  let body = {
    nama,harga,kategori
  }
  let res = await fetch(`${domain}/api/add`,{
    method:"post",
    body:JSON.stringify(body),
    headers:{'Content-Type':'application/json'}
  })
  if (res.status != 200) return false
  return true
}


export async function edit(id:number,nama:string,harga:number) {
  //post request
    let body = {
    nama,harga,id
  }
  let res = await fetch(`${domain}/api/edit`,{
    method:"post",
    body:JSON.stringify(body),
    headers:{'Content-Type':'application/json'}
  })
  if (res.status != 200) return false
  return true
}



