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
  public function get_name($id) {
        // $id contains the dynamic value from the URL segment
        // Use $id to process and respond to the request
        // For example, retrieve data based on $id and return it as JSON
        $data = array('name'=> $id);
        $this->output
            ->set_content_type('application/json')
            ->set_output(json_encode($data));
  }
  public function post() {
        // Check if the request is a POST request
        if ($this->input->server('REQUEST_METHOD') === 'POST') {
            // Retrieve the JSON data from the request body
            $input_data = json_decode(file_get_contents('php://input'));

            if ($input_data !== null) {
                // Process the JSON data
                // For example, update data in the database with $input_data

                // Respond with a JSON success message
                $response = array('message' => 'Data updated successfully');
                $this->output
                    ->set_content_type('application/json')
                    ->set_output(json_encode($response));
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
