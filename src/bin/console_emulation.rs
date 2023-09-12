#[tokio::main]
async fn main() {
    #[cfg(target_arch = "x86_64")]
    {
        let mut some_memory: Vec<u8> = vec![0; 0x4000];
        some_memory[..invaders::space_invaders_rom::SPACE_INVADERS_ROM.len()]
            .clone_from_slice(&invaders::space_invaders_rom::SPACE_INVADERS_ROM);
        let mut machine = invaders::machine::MachineState::new();
        loop {
            if machine.interrupt_due() {
                machine.do_next_interrupt();
            }
            machine.iterate_processor_state();
        }
    }
}
