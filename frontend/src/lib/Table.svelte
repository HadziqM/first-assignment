<script lang="ts">
  import { onMount } from "svelte";
  import { gState, type Produk,dummy,refresh,edit,dell,add } from "./global-state";
  import Modal from "./Modal.svelte";

  onMount(()=>{
    refresh()
  })

  let addModal = false
  let editModal = false
  let deleteModal = false


  let prod:Produk[];

  gState.subscribe(e=>{
    prod = e
  })

  //for handling data
  let selected:Produk = dummy

  let add_name:string;
  let add_harga:number;
  let add_category:string;

  let edit_name:string;
  let edit_harga:number;

  async function handleAdd(){
    await add(add_name,add_harga,add_category)
    await refresh()
    addModal = false
  }
  async function handleEdit(){
    await edit(selected.id,edit_name,edit_harga)
    await refresh()
    editModal = false
  }
  async function handleDelete(){
    await dell(selected.id)
    await refresh()
    deleteModal = false
  }
</script>


<div class="add-button">
  <button on:click={()=>addModal=true}>Add Produk</button>
</div>
<table>
  <thead>
    <tr>
      <td>Nama Produk</td>
      <td>Kategori</td>
      <td>Harga</td>
      <td>Edit</td>
      <td>Delete</td>
    </tr>
  </thead>
  <tbody>
    {#each prod as pd}
      <tr>
        <td>{pd.nama}</td>
        <td>{pd.kategori}</td>
        <td>{`Rp.${pd.harga.toLocaleString("id-ID")},00`}</td>
        <td><button on:click={()=>{
          selected = pd
          editModal = true
          }}>✍</button></td>
        <td><button on:click={()=>{
          selected = pd
          deleteModal = true
          }}>💥</button></td>
      </tr>
    {/each}
  </tbody>
</table>

<Modal bind:showModal={addModal}>
  <h3> Add New Produk into database</h3>
  <div class="flex-col">
    <p>nama produk</p>
    <input type="text" bind:value={add_name}>
    <p>harga</p>
    <input type="number" bind:value={add_harga}>
    <p>kategori</p>
    <input type="text" bind:value={add_category}>
    <button class="blue-button m-1" on:click={()=>handleAdd()}>Submit</button>
  </div>
</Modal>

<Modal bind:showModal={editModal}>
  <h3>{`Edit Produk ${selected.nama}`}</h3>
    <div class="flex-col">
    <p>nama produk</p>
    <input type="text" bind:value={edit_name}>
    <p>harga</p>
    <input type="number" bind:value={edit_harga}>
    <button class="blue-button m-1" on:click={()=>handleEdit()}>Submit</button>
  </div>
</Modal>

<Modal bind:showModal={deleteModal}>
  <h3>{`Delete ${selected.nama} from database?`}</h3>
  <div class="flex center">
    <button class="blue-button m-1" on:click={()=>handleDelete()}>Yes</button>
    <button class="red-button m-1" on:click={()=>deleteModal=false}>Cencel</button>
  </div>
</Modal>

<style>
  .add-button {
    margin: 1rem;
  }
  .add-button button {
    background: blue;
  }
</style>
