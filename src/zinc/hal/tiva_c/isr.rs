// Zinc, the bare metal stack for rust.
// Copyright 2014 Lionel Flandrin <lionel@svkt.org>
// Copyright 2014 Ilyas Gasanov <torso.nafi@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::option::{Option, Some, None};

extern {
  fn isr_gpio_port_a();
  fn isr_gpio_port_b();
  fn isr_gpio_port_c();
  fn isr_gpio_port_d();
  fn isr_gpio_port_e();
  fn isr_uart_0();
  fn isr_uart_1();
  fn isr_ssi_0();
  fn isr_i2c_0();
  fn isr_pwm_0_fault();
  fn isr_pwm_0_gen_0();
  fn isr_pwm_0_gen_1();
  fn isr_pwm_0_gen_2();
  fn isr_qei_0();
  fn isr_adc_0_seq_0();
  fn isr_adc_0_seq_1();
  fn isr_adc_0_seq_2();
  fn isr_adc_0_seq_3();
  fn isr_wdt();
  fn isr_timer_0_a();
  fn isr_timer_0_b();
  fn isr_timer_1_a();
  fn isr_timer_1_b();
  fn isr_timer_2_a();
  fn isr_timer_2_b();
  fn isr_aco_0();
  fn isr_aco_1();
  fn isr_aco_2();
  fn isr_system_control();
  fn isr_flash_eeprom();
  fn isr_gpio_port_f();
  fn isr_gpio_port_g();
  fn isr_gpio_port_h();
  fn isr_uart_2();
  fn isr_ssi_1();
  fn isr_timer_3_a();
  fn isr_timer_3_b();
  fn isr_i2c_1();
  fn isr_qei_1();
  fn isr_can_0();
  fn isr_can_1();
  fn isr_hibernation();
  fn isr_usb_0();
  fn isr_pwm_0_gen_3();
  fn isr_udma_software();
  fn isr_udma_error();
  fn isr_adc_1_seq_0();
  fn isr_adc_1_seq_1();
  fn isr_adc_1_seq_2();
  fn isr_adc_1_seq_3();
  fn isr_gpio_port_j();
  fn isr_gpio_port_k();
  fn isr_gpio_port_l();
  fn isr_ssi_2();
  fn isr_ssi_3();
  fn isr_uart_3();
  fn isr_uart_4();
  fn isr_uart_5();
  fn isr_uart_6();
  fn isr_uart_7();
  fn isr_i2c_2();
  fn isr_i2c_3();
  fn isr_timer_4_a();
  fn isr_timer_4_b();
  fn isr_timer_5_a();
  fn isr_timer_5_b();
  fn isr_wtimer_0_a();
  fn isr_wtimer_0_b();
  fn isr_wtimer_1_a();
  fn isr_wtimer_1_b();
  fn isr_wtimer_2_a();
  fn isr_wtimer_2_b();
  fn isr_wtimer_3_a();
  fn isr_wtimer_3_b();
  fn isr_wtimer_4_a();
  fn isr_wtimer_4_b();
  fn isr_wtimer_5_a();
  fn isr_wtimer_5_b();
  fn isr_fpu_exception();
  fn isr_i2c_4();
  fn isr_i2c_5();
  fn isr_gpio_port_m();
  fn isr_gpio_port_n();
  fn isr_qei_2();
  fn isr_gpio_port_p0();
  fn isr_gpio_port_p1();
  fn isr_gpio_port_p2();
  fn isr_gpio_port_p3();
  fn isr_gpio_port_p4();
  fn isr_gpio_port_p5();
  fn isr_gpio_port_p6();
  fn isr_gpio_port_p7();
  fn isr_gpio_port_q0();
  fn isr_gpio_port_q1();
  fn isr_gpio_port_q2();
  fn isr_gpio_port_q3();
  fn isr_gpio_port_q4();
  fn isr_gpio_port_q5();
  fn isr_gpio_port_q6();
  fn isr_gpio_port_q7();
  fn isr_gpio_port_r();
  fn isr_gpio_port_s();
  fn isr_pwm_1_gen_0();
  fn isr_pwm_1_gen_1();
  fn isr_pwm_1_gen_2();
  fn isr_pwm_1_gen_3();
  fn isr_pwm_1_fault();
}

const ISR_COUNT: uint = 139;

#[link_section=".isr_vector_nvic"]
#[no_mangle]
pub static NVIC_VECTOR: [Option<unsafe extern fn()>, ..ISR_COUNT] = [
  Some(isr_gpio_port_a),       //   0  GPIO Port A
  Some(isr_gpio_port_b),       //   1  GPIO Port B
  Some(isr_gpio_port_c),       //   2  GPIO Port C
  Some(isr_gpio_port_d),       //   3  GPIO Port D
  Some(isr_gpio_port_e),       //   4  GPIO Port E
  Some(isr_uart_0),            //   5  UART0 Rx and Tx
  Some(isr_uart_1),            //   6  UART1 Rx and Tx
  Some(isr_ssi_0),             //   7  SSI0 Rx and Tx
  Some(isr_i2c_0),             //   8  I2C0 Master and Slave
  Some(isr_pwm_0_fault),       //   9  PWM 0 Fault
  Some(isr_pwm_0_gen_0),       //  10  PWM 0 Generator 0
  Some(isr_pwm_0_gen_1),       //  11  PWM 0 Generator 1
  Some(isr_pwm_0_gen_2),       //  12  PWM 0 Generator 2
  Some(isr_qei_0),             //  13  Quadrature Encoder 0
  Some(isr_adc_0_seq_0),       //  14  ADC Sequence 0
  Some(isr_adc_0_seq_1),       //  15  ADC Sequence 1
  Some(isr_adc_0_seq_2),       //  16  ADC Sequence 2
  Some(isr_adc_0_seq_3),       //  17  ADC Sequence 3
  Some(isr_wdt),               //  18  Watchdog timer
  Some(isr_timer_0_a),         //  19  Timer 0 subtimer A
  Some(isr_timer_0_b),         //  20  Timer 0 subtimer B
  Some(isr_timer_1_a),         //  21  Timer 1 subtimer A
  Some(isr_timer_1_b),         //  22  Timer 1 subtimer B
  Some(isr_timer_2_a),         //  23  Timer 2 subtimer A
  Some(isr_timer_2_b),         //  24  Timer 2 subtimer B
  Some(isr_aco_0),             //  25  Analog Comparator 0
  Some(isr_aco_1),             //  26  Analog Comparator 1
  Some(isr_aco_2),             //  27  Analog Comparator 2
  Some(isr_system_control),    //  28  System Control (PLL, OSC, BO)
  Some(isr_flash_eeprom),      //  29  Flash and EEPROM Control
  Some(isr_gpio_port_f),       //  30  GPIO Port F
  Some(isr_gpio_port_g),       //  31  GPIO Port G
  Some(isr_gpio_port_h),       //  32  GPIO Port H
  Some(isr_uart_2),            //  33  UART2 Rx and Tx
  Some(isr_ssi_1),             //  34  SSI1 Rx and Tx
  Some(isr_timer_3_a),         //  35  Timer 3 subtimer A
  Some(isr_timer_3_b),         //  36  Timer 3 subtimer B
  Some(isr_i2c_1),             //  37  I2C1 Master and Slave
  Some(isr_qei_1),             //  38  Quadrature Encoder 1
  Some(isr_can_0),             //  39  CAN0
  Some(isr_can_1),             //  40  CAN1
  None,                        //  41  Reserved
  None,                        //  42  Reserved
  Some(isr_hibernation),       //  43  Hibernation Module
  Some(isr_usb_0),             //  44  USB0
  Some(isr_pwm_0_gen_3),       //  45  PWM 0 Generator 3
  Some(isr_udma_software),     //  46  uDMA Software Transfer
  Some(isr_udma_error),        //  47  uDMA Error
  Some(isr_adc_1_seq_0),       //  48  ADC1 Sequence 0
  Some(isr_adc_1_seq_1),       //  49  ADC1 Sequence 1
  Some(isr_adc_1_seq_2),       //  50  ADC1 Sequence 2
  Some(isr_adc_1_seq_3),       //  51  ADC1 Sequence 3
  None,                        //  52  Reserved
  None,                        //  53  Reserved
  Some(isr_gpio_port_j),       //  54  GPIO Port J
  Some(isr_gpio_port_k),       //  55  GPIO Port K
  Some(isr_gpio_port_l),       //  56  GPIO Port L
  Some(isr_ssi_2),             //  57  SSI2 Rx and Tx
  Some(isr_ssi_3),             //  58  SSI3 Rx and Tx
  Some(isr_uart_3),            //  59  UART3 Rx and Tx
  Some(isr_uart_4),            //  60  UART4 Rx and Tx
  Some(isr_uart_5),            //  61  UART5 Rx and Tx
  Some(isr_uart_6),            //  62  UART6 Rx and Tx
  Some(isr_uart_7),            //  63  UART7 Rx and Tx
  None,                        //  64  Reserved
  None,                        //  65  Reserved
  None,                        //  66  Reserved
  None,                        //  67  Reserved
  Some(isr_i2c_2),             //  68  I2C2 Master and Slave
  Some(isr_i2c_3),             //  69  I2C3 Master and Slave
  Some(isr_timer_4_a),         //  70  Timer 4 subtimer A
  Some(isr_timer_4_b),         //  71  Timer 4 subtimer B
  None,                        //  72  Reserved
  None,                        //  73  Reserved
  None,                        //  74  Reserved
  None,                        //  75  Reserved
  None,                        //  76  Reserved
  None,                        //  77  Reserved
  None,                        //  78  Reserved
  None,                        //  79  Reserved
  None,                        //  80  Reserved
  None,                        //  81  Reserved
  None,                        //  82  Reserved
  None,                        //  83  Reserved
  None,                        //  84  Reserved
  None,                        //  85  Reserved
  None,                        //  86  Reserved
  None,                        //  87  Reserved
  None,                        //  88  Reserved
  None,                        //  89  Reserved
  None,                        //  90  Reserved
  None,                        //  91  Reserved
  Some(isr_timer_5_a),         //  92  Timer 5 subtimer A
  Some(isr_timer_5_b),         //  93  Timer 5 subtimer B
  Some(isr_wtimer_0_a),        //  94  Wide Timer 0 subtimer A
  Some(isr_wtimer_0_b),        //  95  Wide Timer 0 subtimer B
  Some(isr_wtimer_1_a),        //  96  Wide Timer 1 subtimer A
  Some(isr_wtimer_1_b),        //  97  Wide Timer 1 subtimer B
  Some(isr_wtimer_2_a),        //  98  Wide Timer 2 subtimer A
  Some(isr_wtimer_2_b),        //  99  Wide Timer 2 subtimer B
  Some(isr_wtimer_3_a),        // 100  Wide Timer 3 subtimer A
  Some(isr_wtimer_3_b),        // 101  Wide Timer 3 subtimer B
  Some(isr_wtimer_4_a),        // 102  Wide Timer 4 subtimer A
  Some(isr_wtimer_4_b),        // 103  Wide Timer 4 subtimer B
  Some(isr_wtimer_5_a),        // 104  Wide Timer 5 subtimer A
  Some(isr_wtimer_5_b),        // 105  Wide Timer 5 subtimer B
  Some(isr_fpu_exception),     // 106  FPU, System Exception (imprecise)
  None,                        // 107  Reserved
  None,                        // 108  Reserved
  Some(isr_i2c_4),             // 109  I2C4 Master and Slave
  Some(isr_i2c_5),             // 110  I2C5 Master and Slave
  Some(isr_gpio_port_m),       // 111  GPIO Port M
  Some(isr_gpio_port_n),       // 112  GPIO Port N
  Some(isr_qei_2),             // 113  Quadrature Encoder 2
  None,                        // 114  Reserved
  None,                        // 115  Reserved
  Some(isr_gpio_port_p0),      // 116  GPIO Port P (Summary or P0)
  Some(isr_gpio_port_p1),      // 117  GPIO Port P1
  Some(isr_gpio_port_p2),      // 118  GPIO Port P2
  Some(isr_gpio_port_p3),      // 119  GPIO Port P3
  Some(isr_gpio_port_p4),      // 120  GPIO Port P4
  Some(isr_gpio_port_p5),      // 121  GPIO Port P5
  Some(isr_gpio_port_p6),      // 122  GPIO Port P6
  Some(isr_gpio_port_p7),      // 123  GPIO Port P7
  Some(isr_gpio_port_q0),      // 124  GPIO Port Q (Summary or Q0)
  Some(isr_gpio_port_q1),      // 125  GPIO Port Q1
  Some(isr_gpio_port_q2),      // 126  GPIO Port Q2
  Some(isr_gpio_port_q3),      // 127  GPIO Port Q3
  Some(isr_gpio_port_q4),      // 128  GPIO Port Q4
  Some(isr_gpio_port_q5),      // 129  GPIO Port Q5
  Some(isr_gpio_port_q6),      // 130  GPIO Port Q6
  Some(isr_gpio_port_q7),      // 131  GPIO Port Q7
  Some(isr_gpio_port_r),       // 132  GPIO Port R
  Some(isr_gpio_port_s),       // 133  GPIO Port S
  Some(isr_pwm_1_gen_0),       // 134  PWM 1 Generator 0
  Some(isr_pwm_1_gen_1),       // 135  PWM 1 Generator 1
  Some(isr_pwm_1_gen_2),       // 136  PWM 1 Generator 2
  Some(isr_pwm_1_gen_3),       // 137  PWM 1 Generator 3
  Some(isr_pwm_1_fault),       // 138  PWM 1 Fault
];
