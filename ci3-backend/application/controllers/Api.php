<?php
defined('BASEPATH') OR exit('No direct script access allowed');

class Api extends CI_Controller {

  public function index() {
    $this->load->database();

    if ($this->db->initialize()) {
            echo 'Database connection is successful!';
        } else {
            echo 'Unable to connect to the database.';
    }

        // $data = array('name' => 'chatgpt');
        // $this->output
        //     ->set_content_type('application/json')
        //     ->set_output(json_encode($data));
  }
  public  function list() {
    $this->load->database();
    $this->db->select('produk.name as nama, produk.harga as harga, kategori.name as kategori, produk.id as id');
    $this->db->from('produk');
    $this->db->join('kategori', 'kategori.id = produk.kategori_id');
    $this->db->join('status', 'status.id = produk.status_id');
    $this->db->where('status.name', 'bisa dijual');
    $query = $this->db->get();

    if ($query->num_rows() > 0) {
        $result = $query->result();
        $this->output
            ->set_content_type('application/json')
            ->set_output(json_encode($result));

    }
  }
  public function delete($id) {
    $this->load->database();
    $this->db->where('id',$id);
    $this->db->delete('produk');
        $this->output
            ->set_status_header(200);
  }

  public function edit() {
    if ($this->input->server('REQUEST_METHOD') === 'POST') {
      $data = json_decode(file_get_contents('php://input'));
      if ($data !== null) {
        $this->load->database();
        $this->db->where('id',$data->id);
        $table = array(
           'name' => $data->nama,
           'harga' => $data->harga
        );
        $this->db->update('produk',$table);
          $this->output
            ->set_status_header(200);

        } else {
            // Invalid JSON data
            $this->output
                ->set_status_header(400)
                ->set_output('Invalid JSON data');
        }
    } else {
        // Invalid request method
        $this->output
            ->set_status_header(405)
            ->set_output('Method not allowed');
    }

  }

  public function add() {
    if ($this->input->server('REQUEST_METHOD') === 'POST') {
      $data = json_decode(file_get_contents('php://input'));
      if ($data !== null) {
        $this->load->database();
        $this->db->select('id')->from('kategori')->where('name', $data->kategori)->limit(1);
        $query = $this->db->get();
        $kategori_id = $query->row()->id;
        $query_data = array(
          'name' => $data->nama,
          'harga' => $data->harga,
          'kategori_id' => $kategori_id,
          'status_id' => 9
        );
        $this->db->insert('produk', $query_data);

        $this->output
            ->set_status_header(200);
        } else {
              // Invalid JSON data
              $this->output
                  ->set_status_header(400)
                  ->set_output('Invalid JSON data');
          }
      } else {
          // Invalid request method
          $this->output
              ->set_status_header(405)
              ->set_output('Method not allowed');
    }

  }

}
