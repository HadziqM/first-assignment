
<?php
// application/core/MY_Config.php

class MY_Config extends CI_Config {

    public function load($file = '', $use_sections = FALSE, $fail_gracefully = FALSE, $_module = NULL) {
        if ($file === 'database') {
            $config = json_decode(file_get_contents(APPPATH . '../../backend-rust/config.json'), true);
            return $config['database'];
        }
        return parent::load($file, $use_sections, $fail_gracefully, $_module);
    }
}
