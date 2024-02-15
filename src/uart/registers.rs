use hal_macros_derive::make_device;

/// # UART Register Offsets
/// See Max 78000 User Guide Page 180, Table 12-7.
mod uro {
    /// # UART Control Register
    pub const UART_CTRL: usize = 0x0000;
    /// # UART Status Register
    pub const UART_STATUS: usize = 0x0004;
    /// # UART Interrupt Enable Register
    pub const UART_INT_EN: usize = 0x0008;
    /// # UART Interrupt Flag Register
    pub const UART_INTERRUPT_FL: usize = 0x000c;
    /// # UART Clock Divisor Register
    pub const UART_CLKDIV: usize = 0x0010;
    /// # UART Oversampling Control Register
    pub const UART_OSR: usize = 0x0014;
    /// # UART Transmit FIFO
    pub const UART_TXPEEK: usize = 0x0018;
    /// # UART Pin Control Register
    pub const UART_PNR: usize = 0x001c;
    /// # UART FIFO Data Register
    pub const UART_FIFO: usize = 0x0020;
    /// # UART DMA Control Register
    pub const UART_DMA: usize = 0x0030;
    /// # UART Wakeup Interrupt Enable Register
    pub const UART_WKEN: usize = 0x0034;
    /// # UART Wakeup Interrupt Flag Register
    pub const UART_WKFL: usize = 0x0038;
}

make_device! {
    /// # UART Control Register
    /// The UART control register. See Page 180, Table 12-8.
    /// # Receive Dual Edge Sampling
    /// This feature can **only** be used with `LPUART`
    /// Can choose to sample only on the rising edge, or both the rising and falling edges.
    /// - 0: Only rising edge
    /// - 1: Both edges
    #[bit(22, RW, uro::UART_CTRL)]
    rx_dual_edge_sampling,


    /// # Fractional Division Mode
    /// This feature can **only** be used with `LPUART`
    /// Can choose to enable fractional baud rate divisor
    /// - 0: Integer baud rate
    /// - 1: 0.5 division resolution
    #[bit(21, RW, uro::UART_CTRL)]
    fractional_divison_mode,


    /// # Clock Auto Gating Mode
    /// Choose to use no auto gating, or to pause UART clock during idle states
    /// *NOTE:* Software should set this to 1
    /// - 0: No gating
    /// - 1: Clock paused during idle states
    #[bit(20, RW, uro::UART_CTRL)]
    clock_auto_gating,


    /// Baud Clock Ready
    /// Check if baud clock is ready
    /// - 0: Baud clock is not ready
    /// - 1: Baud clock is ready
    #[bit(19, RO, uro::UART_CTRL)]
    is_baud_clock_ready,


    /// # Bit Frame Error Detection Enable
    /// Enable or disable frame error detection
    /// This feature can **only** be used with `LPUART`
    /// - 0: Error detection disabled
    /// - 1: Error detection enabled
    #[bit(18, RW, uro::UART_CTRL)]
    bit_frame_error_detection,


    /// # Baud Clock Source
    /// Select the source for the baud generator (See Table 12-1)
    /// - 0: Clock 0
    /// - 1: Clock 1
    /// - 2: Clock 2
    /// - 3: Clock 3
    #[bit(16..=17, RW u8, uro::UART_CTRL)]
    baud_clock_source,


    /// # Baud Clock Enable
    /// Choose if the baud clock is enabled or not
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(15, RW, uro::UART_CTRL)]
    baud_clock_enable,


    /// # Hardware Flow Control RTS `Deassert` Condition.
    /// Describes the conditions when RTS is deasserted
    /// - 0: When FIFO level = C_RX_FIFO_DEPTH, RTS is deasserted
    /// - 1: When FIFO level `>=` UART_CTRL.rx_thd_val, RTS is deasserted
    #[bit(14, WO, uro::UART_CTRL)]
    hardware_flow_rts_deassert_condition,


    /// # Hardware Flow Control
    /// Choose if hardware flow control is enabled, or disabled
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(13, RW, uro::UART_CTRL)]
    hardware_flow_control,


    /// # Number of stop bits
    /// The number of stop bits
    /// - 0: 1 stop bit
    /// - 1: 1.5 stop bit for 5 bit mode, 2 bit mode otherwise
    #[bit(12, RW, uro::UART_CTRL)]
    number_of_stop_bits,


    /// # Character Length
    /// The number of bits in a character in an UART frame.
    /// - 0: 5 bits
    /// - 1: 6 bits
    /// - 2: 7 bits
    /// - 3: 8 bits
    #[bit(10..=11, RW u8, uro::UART_CTRL)]
    character_length,


    /// # Activate Receive FIFO Flush
    /// Write a 1 to flush the receive FIFO
    #[bit(9, RESET, uro::UART_CTRL)]
    activate_receive_fifo_flush,


    /// # Activate Transmit FIFO Flush
    /// Write a 1 to flush the transmit FIFO
    #[bit(8, RESET, uro::UART_CTRL)]
    activiate_transmit_fifo_flush,


    /// # `CTS` Sampling Disable
    /// Choose to enable or disable `CTS` (Clear To Send)
    /// - 0: Enabled
    /// - 1: Disabled
    #[bit(7, RW, uro::UART_CTRL)]
    cts_sampling_disable,


    /// # Parity Value
    /// The parity calculation uses 1s or 0s in data frame
    /// - 0: Use 1s
    /// - 1: Use 0s
    #[bit(6, RW, uro::UART_CTRL)]
    parity_value,


    /// # Parity Odd Even Select
    /// parity to ensure even or odd
    /// - 0: Even parity (default)
    /// - 1: Odd parity
    #[bit(5, RW, uro::UART_CTRL)]
    parity_odd_even,


    /// # Transmit Parity Generation Enable
    /// Use parity for outward transmissions
    /// - 0: Disable parity
    /// - 1: Use parity (placed after data frame)
    #[bit(4, RW, uro::UART_CTRL)]
    transmit_parity_generation_enable,


    /// # Receive FIFO Threshold
    /// The byte size of FIFO before CPU interrupt is sent
    /// ```
    /// Note: Setting threshold too low at high speeds can slow CPU
    /// and cause loss of data
    /// ```
    /// - 0: Reserved
    /// - 1: 1 byte
    /// - 2: 2 bytes
    /// - 3: 3 bytes
    /// - 4: 4 bytes
    /// - 5: 5 bytes
    /// - 6: 6 bytes
    /// - 7: 7 bytes
    /// - 8: 8 bytes
    /// - 9-15: Reserved
    #[bit(0..=3, RW u8, uro::UART_CTRL)]
    recieve_fifo_threshold,
}

make_device! {
    /// # UART Status Register
    /// The UART Status Register. See page 182, table 12-9
    /// # Transmit FIFO Level
    /// Checks # of bytes in outbound FIFO buffer
    /// - 0-8: Current # of bytes in buffer
    /// - 9-15: Reserved
    #[bit(12..=15, RO u8, uro::UART_STATUS)]
    get_transmit_fifo_level,


    /// # Receive FIFO Level
    /// Checks # of bytes in inbound FIFO buffer
    /// - 0-8: Current # of bytes in buffer
    /// - 9-15: Reserved
    #[bit(8..=11, RO u8, uro::UART_STATUS)]
    get_receive_fifo_level,


    /// # Transmit FIFO Full
    /// Checks if the outbound data buffer has filled up
    /// - 0: Not full
    /// - 1: Full
    #[bit(7, RO, uro::UART_STATUS)]
    is_transmit_fifo_full,


    /// # Transmit FIFO Empty
    /// Checks if the outbound data buffer is empty
    /// - 0: Not empty
    /// - 1: Empty
    #[bit(6, RO, uro::UART_STATUS)]
    is_transmit_fifo_empty,


    /// # Receive FIFO Full
    /// Checks if the inbound data buffer has filled up
    /// - 0: Not full
    /// - 1: Full
    #[bit(5, RO, uro::UART_STATUS)]
    is_receive_fifo_full,


    /// # Receive FIFO Empty
    /// Checks if the inbound data buffer is empty
    /// - 0: Not empty
    /// - 1: Empty
    #[bit(4, RO, uro::UART_STATUS)]
    is_receive_fifo_empty,


    /// # Receive Busy
    /// Checks if the inbound data line is busy
    /// - 0: Not busy
    /// - 1: Busy
    #[bit(1, RO, uro::UART_STATUS)]
    is_receive_busy,


    /// # Transmit Busy
    /// Checks if the outbound data line is busy
    /// - 0: Not busy
    /// - 1: Busy
    #[bit(0, RO, uro::UART_STATUS)]
    is_transmit_busy,
}

/// # UART Interrupt Enable Register
/// The UART Interrupt Enable Register. See page 182, table 12-10
pub struct InterruptEnableRegister<const PORT_PTR: usize> {}
reg_impl!(RW, InterruptEnableRegister, uro::UART_INT_EN);

impl<const PORT_PTR: usize> InterruptEnableRegister<PORT_PTR> {
    bit_impl! {6, RW,
    /// # Set Transmit FIFO Half-Empty Event Interrupt Enable
    /// Sets whether the interrupt for half-full outbound FIFO buffer is enabled
    /// - 0: Disabled
    /// - 1: Enabled
    set_transmit_fifo_half_empty_event,
    /// # Get Transmit FIFO Half-Empty Event Interrupt Enable
    /// Gets whether the interrupt for half-full outbound FIFO buffer is enabled
    /// - 0: Disabled
    /// - 1: Enabled
    get_transmit_fifo_half_empty_event}

    bit_impl! {4, RW,
    /// # Set Receive FIFO Half-Empty Event Interrupt Enable
    /// Sets whether the interrupt for half-full inbound FIFO buffer is enabled
    /// - 0: Disabled
    /// - 1: Enabled
    set_receive_fifo_half_empty_even,
    /// # Get Receive FIFO Half-Empty Event Interrupt Enable
    /// Gets whether the interrupt for half-full inbound FIFO buffer is enabled
    /// - 0: Disabled
    /// - 1: Enabled
    get_receive_fifo_half_empty_even}

    bit_impl! {3, RW,
    /// # Set Receive FIFO Threshold Event Interrupt Enable
    /// Sets whether the interrupt for filled inbound FIFO buffer is enabled
    /// - 0: Disabled
    /// - 1: Enabled
    set_receive_fifo_thershold_event,
    /// # Get Receive FIFO Threshold Event Interrupt Enable
    /// Gets whether the interrupt for filled inbound FIFO buffer is enabled
    /// - 0: Disabled
    /// - 1: Enabled
    get_receive_fifo_thershold_event}

    bit_impl! {2, RW,
    /// # Set `CTS` Signal Change Event Interrupt Enable
    /// Sets if the interrupt for a change in CTS Signal is enabled
    /// - 0: Disabled
    /// - 1: Enabled
    set_cts_signal_change_event,
    /// # Get `CTS` Signal Change Event Interrupt Enable
    /// Gets if the interrupt for a change in CTS Signal is enabled
    /// - 0: Disabled
    /// - 1: Enabled
    get_cts_signal_change_event}

    bit_impl! {1, RW,
    /// # Set Receive Parity Event Interrupt Enable
    /// Set if parity errors on received data is enabled
    /// - 0: Disabled
    /// - 1: Enabled
    set_receive_parity_event,
    /// # Get Receive Parity Event Interrupt Enable
    /// Get if parity errors on received data is enabled
    /// - 0: Disabled
    /// - 1: Enabled
    get_receive_parity_event}

    bit_impl! {0, RW,
    /// # Set Receive Frame Error Event Interrupt Enable
    /// Set if stop bit not being recognized generates an interrupt
    /// - 0: Disabled
    /// - 1: Enabled
    set_receive_frame_error_event,
    /// # Get Receive Frame Error Event Interrupt Enable
    /// Get if stop bit not being recognized generates an interrupt
    /// - 0: Disabled
    /// - 1: Enabled
    get_receive_frame_error_event}
}

make_device! {
    device_ports(mmio::UART_0, mmio::UART_1,
    mmio::UART_2)
    /// UART Interrupt Flag Register
    /// The UART Interrupt Flag Register. See Page 183, Table 12-11.
    /// # Get Transmit FIFO Half-Empty Interrupt Flag
    /// Get the status of the transmit FIFO half-empty flag
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(6, RW1C, uro::UART_INTERRUPT_FL)]
    get_transmit_fifo_half_empty_interrupt_flag,


    /// # Get Receive FIFO Threshold Interrupt Flag
    /// Get the status flag for the FIFO-filled flag
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(4, RW1C, uro::UART_INTERRUPT_FL)]
    get_receive_fifo_threshold_interrupt_flag,


    /// # Get Receive FIFO Overrun Interrupt Flag
    /// Get the status flag for the inbound FIFO buffer overrun flag
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(3, RW1C, uro::UART_INTERRUPT_FL)]
    get_receive_fifo_overrun_interrupt_flag,


    /// # Get CTS Signal Change Interrupt Flag
    /// The status flag for changes in CTS Signal
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(2, RW1C, uro::UART_INTERRUPT_FL)]
    get_cts_signal_change_interrupt_flag,


    /// # Get Receive Parity Error Interrupt Flag
    /// The status flag for errors in the received parity bit
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(1, RW1C, uro::UART_INTERRUPT_FL)]
    get_receive_parity_error_interrupt_flag,


    /// # Get Receive Frame Error Interrupt Flag
    /// The status flag for errors in the received parity bit
    /// - 0: Disabled
    /// - 1: Enabled
    #[bit(0, RW1C, uro::UART_INTERRUPT_FL)]
    get_receive_frame_error_interrupt_flag,
}

/// # UART Clock Divisor Register
/// The UART Clock Divisor Register. See Page 183-184, Table 12-12
pub struct ClockDivisorRegister<const PORT_PTR: usize> {}
reg_impl!(RW, ClockDivisorRegister, uro::UART_CLKDIV);

impl<const PORT_PTR: usize> ClockDivisorRegister<PORT_PTR> {
    bit_impl! {0..=19, RW u32,
    /// # Get Baud Rate Divisor
    get_baud_rate_divisor,
    /// # Set Baud Rate Divisor
    set_baud_rate_divisor}
}

/// # UART Oversampling Control Register
/// The UART Oversampling Control Register. See Page 184, Table 12-13
pub struct OversamplingControlRegister<const PORT_PTR: usize> {}
reg_impl!(RW, OversamplingControlRegister, uro::UART_OSR);

impl<const PORT_PTR: usize> OversamplingControlRegister<PORT_PTR> {
    bit_impl! {0..=2, RW u8,
    /// # Get LPUART Over Sampling Rate
    get_lpuart_oversampling_rate,
    /// # Set LPUART Over Sampling Rate
    set_lpuart_oversampling_rate}
}

/// # UART Transmit FIFO Register
/// The UART Transmit FIFO Register. See Page 184, Table 12-14.
pub struct TransmitFIFORegister<const PORT_PTR: usize> {}
reg_impl!(RO, TransmitFIFORegister, uro::UART_TXPEEK);

impl<const PORT_PTR: usize> TransmitFIFORegister<PORT_PTR> {
    bit_impl! {0..=7, RO u8,
    /// Get Transmit FIFO Data
    get_transmit_fifo_data}
}

/// # UART Pin Control Register
/// The UART Pin Control Register. See Page 184-185, Table 12-15.
pub struct PinControlRegister<const PORT_PTR: usize> {}
reg_impl!(RW, PinControlRegister, uro::UART_PNR);

impl<const PORT_PTR: usize> PinControlRegister<PORT_PTR> {
    bit_impl! {1, RW,
    /// # Get RTS Output State
    get_rts_output_state,
    /// # Set RTS Output State
    set_rts_output_state}

    bit_impl! {0, RO,
    /// # Get CTS Pin State
    get_cts_pin_state}
}

/// # UART Data Register
/// The UART Data Register. See Page 185, Table 12-16.
pub struct DataRegister<const PORT_PTR: usize> {}
reg_impl!(RW, DataRegister, uro::UART_FIFO);

impl<const PORT_PTR: usize> DataRegister<PORT_PTR> {
    bit_impl! {8, RO,
    /// # Get Receive FIFO Byte Parity
    get_receive_fifo_byte_parity}

    bit_impl! {0..=7, RW u8,
    /// # Get Transmit/Receive FIFO Data
    get_transmit_receive_fifo_data,
    /// # Set Transmit/Receive FIFO Data
    set_transmit_receive_fifo_data}
}

/// # UART DMA Register
/// The UART DMA Register. See Page 185, Table 12-17.
pub struct DMARegister<const PORT_PTR: usize> {}
reg_impl!(RW, DMARegister, uro::UART_DMA);

impl<const PORT_PTR: usize> DMARegister<PORT_PTR> {
    bit_impl! {9, RW,
    /// # Set Receive DMA Channel Enable
    /// The documentation has a typo for this bit's access.
    /// It says "0" while it should say "R/W".
    set_receive_dma_channel_enable,
    /// # Get Receive DMA Channel Enable
    is_receive_dma_channel_enable}

    bit_impl! {5..=8, RW u8,
    /// # Set Receive FIFO Level DMA Threshold
    /// The documentation has a typo for this bit's access.
    /// It says "0" while it should say "R/W".
    set_receive_fifo_level_dma_threshold,
    /// # Get Receive FIFO Level DMA Threshold
    get_receive_fifo_level_dma_threshold}

    bit_impl! {4, RW,
    /// # Set Transmit DMA Channel Enable
    set_transmit_dma_channel_enable,
    /// # Get Transmit DMA Channel Enable
    get_transmit_dma_channel_enable}

    bit_impl! {0..=3, RW u8,
    /// # Set Transmit FIFO Level DMA Threshold
    set_transmit_dma_level_dma_threshold,
    /// # Get Transmit FIFO Level DMA Threshold
    get_transmit_dma_level_dma_threshold}
}

/// # UART Wakeup Enable
/// The UART Wakeup Enable Register. See Page 185-186, Table 12-18.
pub struct WakeupEnableRegister<const PORT_PTR: usize> {}
reg_impl!(RW, WakeupEnableRegister, uro::UART_WKEN);

impl<const PORT_PTR: usize> WakeupEnableRegister<PORT_PTR> {
    bit_impl! {2, RW,
    /// # Set Receive FIFO Threshold Wake-up Event Enable
    set_receive_fifo_threshold_wakeup_event_enable,
    /// # Get Receive FIFO Threshold Wake-up Event Enable
    is_receive_fifo_threshold_wakeup_event_enable}

    bit_impl! {1, RW,
    /// # Receive FIFO Full Wake-up Event Enable
    set_receive_fifo_full_wakeup_event_enable,
    /// # Get Receive FIFO Full Wake-up Event Enable
    get_receive_fifo_full_wakeup_event_enable}

    bit_impl! {0, RW,
    /// # Receive FIFO Not Empty Wake-up Event Enable
    set_receive_fifo_not_empty_wakeup_event_enable,
    /// # Get Receive FIFO Not Empty Wake-up Event Enable
    get_receive_fifo_not_empty_wakeup_event_enable}
}

/// # UART Wakeup Flag Register
/// The UART Wakeup Flag register. See Page 186, Table 12-19.
pub struct WakeupFlagRegister<const PORT_PTR: usize> {}
reg_impl!(RW, WakeupFlagRegister, uro::UART_WKFL);

impl<const PORT_PTR: usize> WakeupFlagRegister<PORT_PTR> {
    bit_impl! {2, RW,
    /// # Set Receive FIFO Threshold Wake-up Event
    set_receive_fifo_threshold_wakeup_event,
    /// # Get Receive FIFO Threshold Wake-up Event
    is_receive_fifo_threshold_wakeup_event}

    bit_impl! {1, RW,
    /// # Receive FIFO Full Wake-up Event
    set_receive_fifo_full_wakeup_event,
    /// # Get Receive FIFO Full Wake-up Event
    get_receive_fifo_full_wakeup_event}

    bit_impl! {0, RW,
    /// # Receive FIFO Not Empty Wake-up Event
    set_receive_fifo_not_empty_wakeup_event,
    /// # Get Receive FIFO Not Empty Wake-up Event
    get_receive_fifo_not_empty_wakeup_event}
}
