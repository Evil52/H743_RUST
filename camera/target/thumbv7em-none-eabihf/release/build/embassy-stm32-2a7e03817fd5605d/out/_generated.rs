embassy_hal_internal::peripherals_definition!(
    PA0,
    PA1,
    PA2,
    PA3,
    PA4,
    PA5,
    PA6,
    PA7,
    PA8,
    PA9,
    PA10,
    PA11,
    PA12,
    PA13,
    PA14,
    PA15,
    PB0,
    PB1,
    PB2,
    PB3,
    PB4,
    PB5,
    PB6,
    PB7,
    PB8,
    PB9,
    PB10,
    PB11,
    PB12,
    PB13,
    PB14,
    PB15,
    PC0,
    PC1,
    PC2,
    PC3,
    PC4,
    PC5,
    PC6,
    PC7,
    PC8,
    PC9,
    PC10,
    PC11,
    PC12,
    PC13,
    PC14,
    PC15,
    PD0,
    PD1,
    PD2,
    PD3,
    PD4,
    PD5,
    PD6,
    PD7,
    PD8,
    PD9,
    PD10,
    PD11,
    PD12,
    PD13,
    PD14,
    PD15,
    PE0,
    PE1,
    PE2,
    PE3,
    PE4,
    PE5,
    PE6,
    PE7,
    PE8,
    PE9,
    PE10,
    PE11,
    PE12,
    PE13,
    PE14,
    PE15,
    PH0,
    PH1,
    ADC1,
    ADC12_COMMON,
    ADC2,
    ADC3,
    ADC3_COMMON,
    BDMA,
    CEC,
    COMP1,
    COMP2,
    CRC,
    CRS,
    DAC1,
    DBGMCU,
    DCMI,
    DLYB_QUADSPI,
    DLYB_SDMMC1,
    DLYB_SDMMC2,
    DMA1,
    DMA2,
    DMA2D,
    DMAMUX1,
    DMAMUX2,
    ETH,
    ETH_SMA,
    FDCAN1,
    FDCAN2,
    FDCANRAM,
    FLASH,
    FMC,
    HRTIM1,
    HSEM,
    I2C1,
    I2C2,
    I2C3,
    I2C4,
    IWDG1,
    JPEG,
    LPTIM1,
    LPTIM2,
    LPTIM3,
    LPTIM4,
    LPTIM5,
    LPUART1,
    LTDC,
    MDIOS,
    MDMA,
    OPAMP1,
    OPAMP2,
    PWR,
    QUADSPI,
    MCO1,
    MCO2,
    RCC,
    RNG,
    RTC,
    SAI1,
    SAI2,
    SAI3,
    SAI4,
    SDMMC1,
    SDMMC2,
    SPDIFRX1,
    SPI1,
    SPI2,
    SPI3,
    SPI4,
    SPI6,
    SYSCFG,
    TIM1,
    TIM12,
    TIM13,
    TIM14,
    TIM15,
    TIM16,
    TIM17,
    TIM2,
    TIM3,
    TIM4,
    TIM5,
    TIM6,
    TIM7,
    TIM8,
    UART4,
    UART5,
    UART7,
    UART8,
    UID,
    USART1,
    USART2,
    USART3,
    USART6,
    USB_OTG_FS,
    USB_OTG_HS,
    VREFBUF,
    WWDG1,
    EXTI0,
    EXTI1,
    EXTI2,
    EXTI3,
    EXTI4,
    EXTI5,
    EXTI6,
    EXTI7,
    EXTI8,
    EXTI9,
    EXTI10,
    EXTI11,
    EXTI12,
    EXTI13,
    EXTI14,
    EXTI15,
    DMA1_CH0,
    DMA1_CH1,
    DMA1_CH2,
    DMA1_CH3,
    DMA1_CH4,
    DMA1_CH5,
    DMA1_CH6,
    DMA1_CH7,
    DMA2_CH0,
    DMA2_CH1,
    DMA2_CH2,
    DMA2_CH3,
    DMA2_CH4,
    DMA2_CH5,
    DMA2_CH6,
    DMA2_CH7,
    BDMA_CH0,
    BDMA_CH1,
    BDMA_CH2,
    BDMA_CH3,
    BDMA_CH4,
    BDMA_CH5,
    BDMA_CH6,
    BDMA_CH7,
    MDMA_CH0,
    MDMA_CH1,
    MDMA_CH2,
    MDMA_CH3,
    MDMA_CH4,
    MDMA_CH5,
    MDMA_CH6,
    MDMA_CH7,
    MDMA_CH8,
    MDMA_CH9,
    MDMA_CH10,
    MDMA_CH11,
    MDMA_CH12,
    MDMA_CH13,
    MDMA_CH14,
    MDMA_CH15
);
embassy_hal_internal::peripherals_struct!(
    PA0,
    PA1,
    PA2,
    PA3,
    PA4,
    PA5,
    PA6,
    PA7,
    PA8,
    PA9,
    PA10,
    PA11,
    PA12,
    PA13,
    PA14,
    PA15,
    PB0,
    PB1,
    PB2,
    PB3,
    PB4,
    PB5,
    PB6,
    PB7,
    PB8,
    PB9,
    PB10,
    PB11,
    PB12,
    PB13,
    PB14,
    PB15,
    PC0,
    PC1,
    PC2,
    PC3,
    PC4,
    PC5,
    PC6,
    PC7,
    PC8,
    PC9,
    PC10,
    PC11,
    PC12,
    PC13,
    PC14,
    PC15,
    PD0,
    PD1,
    PD2,
    PD3,
    PD4,
    PD5,
    PD6,
    PD7,
    PD8,
    PD9,
    PD10,
    PD11,
    PD12,
    PD13,
    PD14,
    PD15,
    PE0,
    PE1,
    PE2,
    PE3,
    PE4,
    PE5,
    PE6,
    PE7,
    PE8,
    PE9,
    PE10,
    PE11,
    PE12,
    PE13,
    PE14,
    PE15,
    PH0,
    PH1,
    ADC1,
    ADC12_COMMON,
    ADC2,
    ADC3,
    ADC3_COMMON,
    BDMA,
    CEC,
    COMP1,
    COMP2,
    CRC,
    CRS,
    DAC1,
    DBGMCU,
    DCMI,
    DLYB_QUADSPI,
    DLYB_SDMMC1,
    DLYB_SDMMC2,
    DMA1,
    DMA2,
    DMA2D,
    DMAMUX1,
    DMAMUX2,
    ETH,
    ETH_SMA,
    FDCAN1,
    FDCAN2,
    FDCANRAM,
    FLASH,
    FMC,
    HRTIM1,
    HSEM,
    I2C1,
    I2C2,
    I2C3,
    I2C4,
    IWDG1,
    JPEG,
    LPTIM1,
    LPTIM2,
    LPTIM3,
    LPTIM4,
    LPTIM5,
    LPUART1,
    LTDC,
    MDIOS,
    MDMA,
    OPAMP1,
    OPAMP2,
    PWR,
    QUADSPI,
    MCO1,
    MCO2,
    RCC,
    RNG,
    RTC,
    SAI1,
    SAI2,
    SAI3,
    SAI4,
    SDMMC1,
    SDMMC2,
    SPDIFRX1,
    SPI1,
    SPI2,
    SPI3,
    SPI4,
    SPI6,
    SYSCFG,
    TIM1,
    TIM12,
    TIM13,
    TIM14,
    TIM15,
    TIM16,
    TIM17,
    TIM3,
    TIM4,
    TIM5,
    TIM6,
    TIM7,
    TIM8,
    UART4,
    UART5,
    UART7,
    UART8,
    UID,
    USART1,
    USART2,
    USART3,
    USART6,
    USB_OTG_FS,
    USB_OTG_HS,
    VREFBUF,
    WWDG1,
    EXTI0,
    EXTI1,
    EXTI2,
    EXTI3,
    EXTI4,
    EXTI5,
    EXTI6,
    EXTI7,
    EXTI8,
    EXTI9,
    EXTI10,
    EXTI11,
    EXTI12,
    EXTI13,
    EXTI14,
    EXTI15,
    DMA1_CH0,
    DMA1_CH1,
    DMA1_CH2,
    DMA1_CH3,
    DMA1_CH4,
    DMA1_CH5,
    DMA1_CH6,
    DMA1_CH7,
    DMA2_CH0,
    DMA2_CH1,
    DMA2_CH2,
    DMA2_CH3,
    DMA2_CH4,
    DMA2_CH5,
    DMA2_CH6,
    DMA2_CH7,
    BDMA_CH0,
    BDMA_CH1,
    BDMA_CH2,
    BDMA_CH3,
    BDMA_CH4,
    BDMA_CH5,
    BDMA_CH6,
    BDMA_CH7,
    MDMA_CH0,
    MDMA_CH1,
    MDMA_CH2,
    MDMA_CH3,
    MDMA_CH4,
    MDMA_CH5,
    MDMA_CH6,
    MDMA_CH7,
    MDMA_CH8,
    MDMA_CH9,
    MDMA_CH10,
    MDMA_CH11,
    MDMA_CH12,
    MDMA_CH13,
    MDMA_CH14,
    MDMA_CH15
);
embassy_hal_internal::interrupt_mod!(
    WWDG,
    PVD_AVD,
    TAMP_STAMP,
    RTC_WKUP,
    FLASH,
    RCC,
    EXTI0,
    EXTI1,
    EXTI2,
    EXTI3,
    EXTI4,
    DMA1_STREAM0,
    DMA1_STREAM1,
    DMA1_STREAM2,
    DMA1_STREAM3,
    DMA1_STREAM4,
    DMA1_STREAM5,
    DMA1_STREAM6,
    ADC,
    FDCAN1_IT0,
    FDCAN2_IT0,
    FDCAN1_IT1,
    FDCAN2_IT1,
    EXTI9_5,
    TIM1_BRK,
    TIM1_UP,
    TIM1_TRG_COM,
    TIM1_CC,
    TIM2,
    TIM3,
    TIM4,
    I2C1_EV,
    I2C1_ER,
    I2C2_EV,
    I2C2_ER,
    SPI1,
    SPI2,
    USART1,
    USART2,
    USART3,
    EXTI15_10,
    RTC_ALARM,
    TIM8_BRK_TIM12,
    TIM8_UP_TIM13,
    TIM8_TRG_COM_TIM14,
    TIM8_CC,
    DMA1_STREAM7,
    FMC,
    SDMMC1,
    TIM5,
    SPI3,
    UART4,
    UART5,
    TIM6_DAC,
    TIM7,
    DMA2_STREAM0,
    DMA2_STREAM1,
    DMA2_STREAM2,
    DMA2_STREAM3,
    DMA2_STREAM4,
    ETH,
    ETH_WKUP,
    FDCAN_CAL,
    DMA2_STREAM5,
    DMA2_STREAM6,
    DMA2_STREAM7,
    USART6,
    I2C3_EV,
    I2C3_ER,
    OTG_HS_EP1_OUT,
    OTG_HS_EP1_IN,
    OTG_HS_WKUP,
    OTG_HS,
    DCMI,
    RNG,
    FPU,
    UART7,
    UART8,
    SPI4,
    SPI5,
    SPI6,
    SAI1,
    LTDC,
    LTDC_ER,
    DMA2D,
    SAI2,
    QUADSPI,
    LPTIM1,
    CEC,
    I2C4_EV,
    I2C4_ER,
    SPDIF_RX,
    OTG_FS_EP1_OUT,
    OTG_FS_EP1_IN,
    OTG_FS_WKUP,
    OTG_FS,
    DMAMUX1_OVR,
    HRTIM1_MASTER,
    HRTIM1_TIMA,
    HRTIM1_TIMB,
    HRTIM1_TIMC,
    HRTIM1_TIMD,
    HRTIM1_TIME,
    HRTIM1_FLT,
    DFSDM1_FLT0,
    DFSDM1_FLT1,
    DFSDM1_FLT2,
    DFSDM1_FLT3,
    SAI3,
    SWPMI1,
    TIM15,
    TIM16,
    TIM17,
    MDIOS_WKUP,
    MDIOS,
    JPEG,
    MDMA,
    SDMMC2,
    HSEM1,
    ADC3,
    DMAMUX2_OVR,
    BDMA_CHANNEL0,
    BDMA_CHANNEL1,
    BDMA_CHANNEL2,
    BDMA_CHANNEL3,
    BDMA_CHANNEL4,
    BDMA_CHANNEL5,
    BDMA_CHANNEL6,
    BDMA_CHANNEL7,
    COMP,
    LPTIM2,
    LPTIM3,
    LPTIM4,
    LPTIM5,
    LPUART1,
    CRS,
    ECC,
    SAI4,
    WAKEUP_PIN,
);
#[cfg(feature = "rt")]
#[interrupt]
fn TIM2() {
    crate::time_driver::get_driver().on_interrupt();
}
pub const MAX_ERASE_SIZE: usize = 131072u32 as usize;
pub mod flash_regions {
    impl crate::flash::FlashBank {
        #[doc = r" Absolute base address."]
        pub fn base(&self) -> u32 {
            match self {
                crate::flash::FlashBank::Bank1 => 134217728u32,
                crate::flash::FlashBank::Bank2 => 135266304u32,
                crate::flash::FlashBank::Otp => panic!("OTP not present"),
            }
        }
    }
    pub const BANK1_REGION: crate::flash::FlashRegion = crate::flash::FlashRegion {
        bank: crate::flash::FlashBank::Bank1,
        offset: 0u32,
        size: 1048576u32,
        erase_size: 131072u32,
        write_size: 32u32,
        erase_value: 255u8,
        _ensure_internal: (),
    };
    #[cfg(flash)]
    pub struct Bank1Region<'d, MODE = crate::flash::Async>(
        pub &'static crate::flash::FlashRegion,
        pub(crate) embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>,
        pub(crate) core::marker::PhantomData<MODE>,
    );
    pub const BANK2_REGION: crate::flash::FlashRegion = crate::flash::FlashRegion {
        bank: crate::flash::FlashBank::Bank2,
        offset: 0u32,
        size: 1048576u32,
        erase_size: 131072u32,
        write_size: 32u32,
        erase_value: 255u8,
        _ensure_internal: (),
    };
    #[cfg(flash)]
    pub struct Bank2Region<'d, MODE = crate::flash::Async>(
        pub &'static crate::flash::FlashRegion,
        pub(crate) embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>,
        pub(crate) core::marker::PhantomData<MODE>,
    );
    #[cfg(flash)]
    pub struct FlashLayout<'d, MODE = crate::flash::Async> {
        pub bank1_region: Bank1Region<'d, MODE>,
        pub bank2_region: Bank2Region<'d, MODE>,
        _mode: core::marker::PhantomData<MODE>,
    }
    #[cfg(flash)]
    impl<'d, MODE> FlashLayout<'d, MODE> {
        pub(crate) fn new(p: embassy_hal_internal::Peri<'d, crate::peripherals::FLASH>) -> Self {
            Self {
                bank1_region: Bank1Region(
                    &BANK1_REGION,
                    unsafe { p.clone_unchecked() },
                    core::marker::PhantomData,
                ),
                bank2_region: Bank2Region(
                    &BANK2_REGION,
                    unsafe { p.clone_unchecked() },
                    core::marker::PhantomData,
                ),
                _mode: core::marker::PhantomData,
            }
        }
    }
    pub const FLASH_REGIONS: [&crate::flash::FlashRegion; 2usize] = [&BANK1_REGION, &BANK2_REGION];
}
impl crate::rcc::SealedRccPeripheral for peripherals::ADC1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d3ccipr().read().adcsel() {
            crate::pac::rcc::vals::Adcsel::PLL2_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC1" , "pll2_p")
            },
            crate::pac::rcc::vals::Adcsel::PLL3_R => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_r . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC1" , "pll3_r")
            },
            crate::pac::rcc::vals::Adcsel::PER => match crate::pac::RCC.d1ccipr().read().persel() {
                crate::pac::rcc::vals::Persel::HSI => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC1" , "hsi")
                },
                crate::pac::rcc::vals::Persel::CSI => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC1" , "csi")
                },
                crate::pac::rcc::vals::Persel::HSE => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC1" , "hse")
                },
                #[allow(unreachable_patterns)]
                _ => panic!(
                    "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                    "ADC1"
                ),
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "ADC1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC1" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((32u8, 5u8)),
            (54u8, 5u8),
            Some(0u8),
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::ADC1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::ADC2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d3ccipr().read().adcsel() {
            crate::pac::rcc::vals::Adcsel::PLL2_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC2" , "pll2_p")
            },
            crate::pac::rcc::vals::Adcsel::PLL3_R => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_r . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC2" , "pll3_r")
            },
            crate::pac::rcc::vals::Adcsel::PER => match crate::pac::RCC.d1ccipr().read().persel() {
                crate::pac::rcc::vals::Persel::HSI => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC2" , "hsi")
                },
                crate::pac::rcc::vals::Persel::CSI => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC2" , "csi")
                },
                crate::pac::rcc::vals::Persel::HSE => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC2" , "hse")
                },
                #[allow(unreachable_patterns)]
                _ => panic!(
                    "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                    "ADC2"
                ),
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "ADC2"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC2" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((32u8, 5u8)),
            (54u8, 5u8),
            Some(0u8),
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::ADC2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::ADC3 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d3ccipr().read().adcsel() {
            crate::pac::rcc::vals::Adcsel::PLL2_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC3" , "pll2_p")
            },
            crate::pac::rcc::vals::Adcsel::PLL3_R => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_r . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC3" , "pll3_r")
            },
            crate::pac::rcc::vals::Adcsel::PER => match crate::pac::RCC.d1ccipr().read().persel() {
                crate::pac::rcc::vals::Persel::HSI => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC3" , "hsi")
                },
                crate::pac::rcc::vals::Persel::CSI => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC3" , "csi")
                },
                crate::pac::rcc::vals::Persel::HSE => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC3" , "hse")
                },
                #[allow(unreachable_patterns)]
                _ => panic!(
                    "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                    "ADC3"
                ),
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "ADC3"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ADC3" , "hclk4")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((34u8, 24u8)),
            (56u8, 24u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::ADC3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::BDMA {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "BDMA" , "hclk4")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "BDMA" , "hclk4")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((34u8, 21u8)),
            (56u8, 21u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::BDMA {}
impl crate::rcc::SealedRccPeripheral for peripherals::CEC {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip2r().read().cecsel() {
            crate::pac::rcc::vals::Cecsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CEC" , "lse")
            },
            crate::pac::rcc::vals::Cecsel::LSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CEC" , "lsi")
            },
            crate::pac::rcc::vals::Cecsel::CSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CEC" , "csi")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "CEC"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CEC" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 27u8)),
            (58u8, 27u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CEC {}
impl crate::rcc::SealedRccPeripheral for peripherals::CRC {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CRC" , "hclk4")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CRC" , "hclk4")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((34u8, 19u8)),
            (56u8, 19u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CRC {}
impl crate::rcc::SealedRccPeripheral for peripherals::CRS {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CRS" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "CRS" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((37u8, 1u8)),
            (59u8, 1u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::CRS {}
impl crate::rcc::SealedRccPeripheral for peripherals::DAC1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DAC1" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DAC1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 29u8)),
            (58u8, 29u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DAC1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::DCMI {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DCMI" , "hclk2")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DCMI" , "hclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((33u8, 0u8)),
            (55u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DCMI {}
impl crate::rcc::SealedRccPeripheral for peripherals::DMA1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA1" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA1" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((32u8, 0u8)),
            (54u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DMA1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::DMA2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA2" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA2" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((32u8, 1u8)),
            (54u8, 1u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DMA2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::DMA2D {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk3 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA2D" , "hclk3")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk3 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "DMA2D" , "hclk3")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((31u8, 4u8)),
            (53u8, 4u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::DMA2D {}
impl crate::rcc::SealedRccPeripheral for peripherals::ETH {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ETH" , "hclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "ETH" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((32u8, 15u8)),
            (54u8, 15u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::ETH {}
impl crate::rcc::SealedRccPeripheral for peripherals::FDCAN1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip1r().read().fdcansel() {
            crate::pac::rcc::vals::Fdcansel::HSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FDCAN1" , "hse")
            },
            crate::pac::rcc::vals::Fdcansel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FDCAN1" , "pll1_q")
            },
            crate::pac::rcc::vals::Fdcansel::PLL2_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FDCAN1" , "pll2_q")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "FDCAN1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FDCAN1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((37u8, 8u8)),
            (59u8, 8u8),
            Some(1u8),
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::FDCAN1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::FDCAN2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip1r().read().fdcansel() {
            crate::pac::rcc::vals::Fdcansel::HSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FDCAN2" , "hse")
            },
            crate::pac::rcc::vals::Fdcansel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FDCAN2" , "pll1_q")
            },
            crate::pac::rcc::vals::Fdcansel::PLL2_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FDCAN2" , "pll2_q")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "FDCAN2"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FDCAN2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((37u8, 8u8)),
            (59u8, 8u8),
            Some(1u8),
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::FDCAN2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::FMC {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d1ccipr().read().fmcsel() {
            crate::pac::rcc::vals::Fmcsel::HCLK3 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hclk3 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FMC" , "hclk3")
            },
            crate::pac::rcc::vals::Fmcsel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FMC" , "pll1_q")
            },
            crate::pac::rcc::vals::Fmcsel::PLL2_R => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_r . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FMC" , "pll2_r")
            },
            crate::pac::rcc::vals::Fmcsel::PER => match crate::pac::RCC.d1ccipr().read().persel() {
                crate::pac::rcc::vals::Persel::HSI => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FMC" , "hsi")
                },
                crate::pac::rcc::vals::Persel::CSI => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FMC" , "csi")
                },
                crate::pac::rcc::vals::Persel::HSE => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FMC" , "hse")
                },
                #[allow(unreachable_patterns)]
                _ => panic!(
                    "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                    "FMC"
                ),
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "FMC"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk3 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "FMC" , "hclk3")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((31u8, 12u8)),
            (53u8, 12u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::FMC {}
impl crate::rcc::SealedRccPeripheral for peripherals::HRTIM1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "HRTIM1" , "pclk2_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "HRTIM1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((38u8, 29u8)),
            (60u8, 29u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::HRTIM1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::HSEM {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "HSEM" , "hclk4")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "HSEM" , "hclk4")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((34u8, 25u8)),
            (56u8, 25u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::HSEM {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip2r().read().i2c1235sel() {
            crate::pac::rcc::vals::I2c1235sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "pclk1")
            },
            crate::pac::rcc::vals::I2c1235sel::PLL3_R => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_r . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "pll3_r")
            },
            crate::pac::rcc::vals::I2c1235sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "hsi")
            },
            crate::pac::rcc::vals::I2c1235sel::CSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "csi")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "I2C1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 21u8)),
            (58u8, 21u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip2r().read().i2c1235sel() {
            crate::pac::rcc::vals::I2c1235sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "pclk1")
            },
            crate::pac::rcc::vals::I2c1235sel::PLL3_R => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_r . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "pll3_r")
            },
            crate::pac::rcc::vals::I2c1235sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "hsi")
            },
            crate::pac::rcc::vals::I2c1235sel::CSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "csi")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "I2C2"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 22u8)),
            (58u8, 22u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C3 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip2r().read().i2c1235sel() {
            crate::pac::rcc::vals::I2c1235sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C3" , "pclk1")
            },
            crate::pac::rcc::vals::I2c1235sel::PLL3_R => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_r . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C3" , "pll3_r")
            },
            crate::pac::rcc::vals::I2c1235sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C3" , "hsi")
            },
            crate::pac::rcc::vals::I2c1235sel::CSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C3" , "csi")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "I2C3"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C3" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 23u8)),
            (58u8, 23u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::I2C4 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d3ccipr().read().i2c4sel() {
            crate::pac::rcc::vals::I2c4sel::PCLK4 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C4" , "pclk4")
            },
            crate::pac::rcc::vals::I2c4sel::PLL3_R => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_r . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C4" , "pll3_r")
            },
            crate::pac::rcc::vals::I2c4sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C4" , "hsi")
            },
            crate::pac::rcc::vals::I2c4sel::CSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C4" , "csi")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "I2C4"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "I2C4" , "pclk4")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((39u8, 7u8)),
            (61u8, 7u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::I2C4 {}
impl crate::rcc::SealedRccPeripheral for peripherals::LPTIM1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip2r().read().lptim1sel() {
            crate::pac::rcc::vals::Lptim1sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "pclk1")
            },
            crate::pac::rcc::vals::Lptim1sel::PLL2_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "pll2_p")
            },
            crate::pac::rcc::vals::Lptim1sel::PLL3_R => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_r . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "pll3_r")
            },
            crate::pac::rcc::vals::Lptim1sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "lse")
            },
            crate::pac::rcc::vals::Lptim1sel::LSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "lsi")
            },
            crate::pac::rcc::vals::Lptim1sel::PER => {
                match crate::pac::RCC.d1ccipr().read().persel() {
                    crate::pac::rcc::vals::Persel::HSI => unsafe {
                        unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "hsi")
                    },
                    crate::pac::rcc::vals::Persel::CSI => unsafe {
                        unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "csi")
                    },
                    crate::pac::rcc::vals::Persel::HSE => unsafe {
                        unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "hse")
                    },
                    #[allow(unreachable_patterns)]
                    _ => panic!(
                        "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                        "LPTIM1"
                    ),
                }
            }
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "LPTIM1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 9u8)),
            (58u8, 9u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop2,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::LPTIM1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::LPTIM2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d3ccipr().read().lptim2sel() {
            crate::pac::rcc::vals::Lptim2sel::PCLK4 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "pclk4")
            },
            crate::pac::rcc::vals::Lptim2sel::PLL2_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "pll2_p")
            },
            crate::pac::rcc::vals::Lptim2sel::PLL3_R => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_r . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "pll3_r")
            },
            crate::pac::rcc::vals::Lptim2sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "lse")
            },
            crate::pac::rcc::vals::Lptim2sel::LSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "lsi")
            },
            crate::pac::rcc::vals::Lptim2sel::PER => {
                match crate::pac::RCC.d1ccipr().read().persel() {
                    crate::pac::rcc::vals::Persel::HSI => unsafe {
                        unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "hsi")
                    },
                    crate::pac::rcc::vals::Persel::CSI => unsafe {
                        unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "csi")
                    },
                    crate::pac::rcc::vals::Persel::HSE => unsafe {
                        unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "hse")
                    },
                    #[allow(unreachable_patterns)]
                    _ => panic!(
                        "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                        "LPTIM2"
                    ),
                }
            }
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "LPTIM2"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM2" , "pclk4")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((39u8, 9u8)),
            (61u8, 9u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop2,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::LPTIM2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::LPTIM3 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM3" , "pclk4")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM3" , "pclk4")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((39u8, 10u8)),
            (61u8, 10u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop2,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::LPTIM3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::LPTIM4 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM4" , "pclk4")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM4" , "pclk4")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((39u8, 11u8)),
            (61u8, 11u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop2,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::LPTIM4 {}
impl crate::rcc::SealedRccPeripheral for peripherals::LPTIM5 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM5" , "pclk4")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPTIM5" , "pclk4")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((39u8, 12u8)),
            (61u8, 12u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop2,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::LPTIM5 {}
impl crate::rcc::SealedRccPeripheral for peripherals::LPUART1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d3ccipr().read().lpuart1sel() {
            crate::pac::rcc::vals::Lpuartsel::PCLK4 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART1" , "pclk4")
            },
            crate::pac::rcc::vals::Lpuartsel::PLL2_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART1" , "pll2_q")
            },
            crate::pac::rcc::vals::Lpuartsel::PLL3_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART1" , "pll3_q")
            },
            crate::pac::rcc::vals::Lpuartsel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART1" , "hsi")
            },
            crate::pac::rcc::vals::Lpuartsel::CSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART1" , "csi")
            },
            crate::pac::rcc::vals::Lpuartsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART1" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "LPUART1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LPUART1" , "pclk4")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((39u8, 3u8)),
            (61u8, 3u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop2,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::LPUART1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::LTDC {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk3 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LTDC" , "pclk3")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk3 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "LTDC" , "pclk3")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((35u8, 3u8)),
            (57u8, 3u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::LTDC {}
impl crate::rcc::SealedRccPeripheral for peripherals::MDIOS {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "MDIOS" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "MDIOS" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((37u8, 5u8)),
            (59u8, 5u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::MDIOS {}
impl crate::rcc::SealedRccPeripheral for peripherals::MDMA {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk3 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "MDMA" , "hclk3")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk3 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "MDMA" , "hclk3")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((31u8, 0u8)),
            (53u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::MDMA {}
impl crate::rcc::SealedRccPeripheral for peripherals::OPAMP1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "OPAMP1" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "OPAMP1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((37u8, 4u8)),
            (59u8, 4u8),
            Some(2u8),
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::OPAMP1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::OPAMP2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "OPAMP2" , "pclk1")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "OPAMP2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((37u8, 4u8)),
            (59u8, 4u8),
            Some(2u8),
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::OPAMP2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::QUADSPI {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d1ccipr().read().quadspisel() {
            crate::pac::rcc::vals::Fmcsel::HCLK3 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hclk3 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "QUADSPI" , "hclk3")
            },
            crate::pac::rcc::vals::Fmcsel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "QUADSPI" , "pll1_q")
            },
            crate::pac::rcc::vals::Fmcsel::PLL2_R => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_r . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "QUADSPI" , "pll2_r")
            },
            crate::pac::rcc::vals::Fmcsel::PER => match crate::pac::RCC.d1ccipr().read().persel() {
                crate::pac::rcc::vals::Persel::HSI => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "QUADSPI" , "hsi")
                },
                crate::pac::rcc::vals::Persel::CSI => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "QUADSPI" , "csi")
                },
                crate::pac::rcc::vals::Persel::HSE => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "QUADSPI" , "hse")
                },
                #[allow(unreachable_patterns)]
                _ => panic!(
                    "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                    "QUADSPI"
                ),
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "QUADSPI"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk3 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "QUADSPI" , "hclk3")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((31u8, 14u8)),
            (53u8, 14u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::QUADSPI {}
impl crate::rcc::SealedRccPeripheral for peripherals::RNG {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip2r().read().rngsel() {
            crate::pac::rcc::vals::Rngsel::HSI48 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi48 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RNG" , "hsi48")
            },
            crate::pac::rcc::vals::Rngsel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RNG" , "pll1_q")
            },
            crate::pac::rcc::vals::Rngsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RNG" , "lse")
            },
            crate::pac::rcc::vals::Rngsel::LSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RNG" , "lsi")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "RNG"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RNG" , "hclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((33u8, 6u8)),
            (55u8, 6u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::RNG {}
impl crate::rcc::SealedRccPeripheral for peripherals::RTC {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.bdcr().read().rtcsel() {
            crate::pac::rcc::vals::Rtcsel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RTC" , "lse")
            },
            crate::pac::rcc::vals::Rtcsel::LSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RTC" , "lsi")
            },
            crate::pac::rcc::vals::Rtcsel::HSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RTC" , "hse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "RTC"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "RTC" , "pclk4")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (61u8, 16u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Standby,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::RTC {}
impl crate::rcc::SealedRccPeripheral for peripherals::SAI1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip1r().read().sai1sel() {
            crate::pac::rcc::vals::Saisel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "pll1_q")
            },
            crate::pac::rcc::vals::Saisel::PLL2_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "pll2_p")
            },
            crate::pac::rcc::vals::Saisel::PLL3_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "pll3_p")
            },
            crate::pac::rcc::vals::Saisel::I2S_CKIN => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . i2s_ckin . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "i2s_ckin")
            },
            crate::pac::rcc::vals::Saisel::PER => match crate::pac::RCC.d1ccipr().read().persel() {
                crate::pac::rcc::vals::Persel::HSI => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "hsi")
                },
                crate::pac::rcc::vals::Persel::CSI => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "csi")
                },
                crate::pac::rcc::vals::Persel::HSE => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "hse")
                },
                #[allow(unreachable_patterns)]
                _ => panic!(
                    "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                    "SAI1"
                ),
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SAI1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((38u8, 22u8)),
            (60u8, 22u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SAI1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SAI2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip1r().read().sai23sel() {
            crate::pac::rcc::vals::Saisel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI2" , "pll1_q")
            },
            crate::pac::rcc::vals::Saisel::PLL2_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI2" , "pll2_p")
            },
            crate::pac::rcc::vals::Saisel::PLL3_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI2" , "pll3_p")
            },
            crate::pac::rcc::vals::Saisel::I2S_CKIN => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . i2s_ckin . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI2" , "i2s_ckin")
            },
            crate::pac::rcc::vals::Saisel::PER => match crate::pac::RCC.d1ccipr().read().persel() {
                crate::pac::rcc::vals::Persel::HSI => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI2" , "hsi")
                },
                crate::pac::rcc::vals::Persel::CSI => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI2" , "csi")
                },
                crate::pac::rcc::vals::Persel::HSE => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI2" , "hse")
                },
                #[allow(unreachable_patterns)]
                _ => panic!(
                    "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                    "SAI2"
                ),
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SAI2"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI2" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((38u8, 23u8)),
            (60u8, 23u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SAI2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SAI3 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip1r().read().sai23sel() {
            crate::pac::rcc::vals::Saisel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI3" , "pll1_q")
            },
            crate::pac::rcc::vals::Saisel::PLL2_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI3" , "pll2_p")
            },
            crate::pac::rcc::vals::Saisel::PLL3_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI3" , "pll3_p")
            },
            crate::pac::rcc::vals::Saisel::I2S_CKIN => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . i2s_ckin . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI3" , "i2s_ckin")
            },
            crate::pac::rcc::vals::Saisel::PER => match crate::pac::RCC.d1ccipr().read().persel() {
                crate::pac::rcc::vals::Persel::HSI => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI3" , "hsi")
                },
                crate::pac::rcc::vals::Persel::CSI => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI3" , "csi")
                },
                crate::pac::rcc::vals::Persel::HSE => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI3" , "hse")
                },
                #[allow(unreachable_patterns)]
                _ => panic!(
                    "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                    "SAI3"
                ),
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SAI3"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI3" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((38u8, 24u8)),
            (60u8, 24u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SAI3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SAI4 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI4" , "pclk4")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SAI4" , "pclk4")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((39u8, 21u8)),
            (61u8, 21u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SAI4 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SDMMC1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d1ccipr().read().sdmmcsel() {
            crate::pac::rcc::vals::Sdmmcsel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDMMC1" , "pll1_q")
            },
            crate::pac::rcc::vals::Sdmmcsel::PLL2_R => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_r . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDMMC1" , "pll2_r")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SDMMC1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk3 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDMMC1" , "hclk3")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((31u8, 16u8)),
            (53u8, 16u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SDMMC1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SDMMC2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d1ccipr().read().sdmmcsel() {
            crate::pac::rcc::vals::Sdmmcsel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDMMC2" , "pll1_q")
            },
            crate::pac::rcc::vals::Sdmmcsel::PLL2_R => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_r . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDMMC2" , "pll2_r")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SDMMC2"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SDMMC2" , "hclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((33u8, 9u8)),
            (55u8, 9u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SDMMC2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPDIFRX1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip1r().read().spdifrxsel() {
            crate::pac::rcc::vals::Spdifrxsel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPDIFRX1" , "pll1_q")
            },
            crate::pac::rcc::vals::Spdifrxsel::PLL2_R => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_r . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPDIFRX1" , "pll2_r")
            },
            crate::pac::rcc::vals::Spdifrxsel::PLL3_R => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_r . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPDIFRX1" , "pll3_r")
            },
            crate::pac::rcc::vals::Spdifrxsel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPDIFRX1" , "hsi")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SPDIFRX1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPDIFRX1" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 16u8)),
            (58u8, 16u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPDIFRX1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip1r().read().spi123sel() {
            crate::pac::rcc::vals::Saisel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI1" , "pll1_q")
            },
            crate::pac::rcc::vals::Saisel::PLL2_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI1" , "pll2_p")
            },
            crate::pac::rcc::vals::Saisel::PLL3_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI1" , "pll3_p")
            },
            crate::pac::rcc::vals::Saisel::I2S_CKIN => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . i2s_ckin . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI1" , "i2s_ckin")
            },
            crate::pac::rcc::vals::Saisel::PER => match crate::pac::RCC.d1ccipr().read().persel() {
                crate::pac::rcc::vals::Persel::HSI => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI1" , "hsi")
                },
                crate::pac::rcc::vals::Persel::CSI => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI1" , "csi")
                },
                crate::pac::rcc::vals::Persel::HSE => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI1" , "hse")
                },
                #[allow(unreachable_patterns)]
                _ => panic!(
                    "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                    "SPI1"
                ),
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SPI1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((38u8, 12u8)),
            (60u8, 12u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip1r().read().spi123sel() {
            crate::pac::rcc::vals::Saisel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI2" , "pll1_q")
            },
            crate::pac::rcc::vals::Saisel::PLL2_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI2" , "pll2_p")
            },
            crate::pac::rcc::vals::Saisel::PLL3_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI2" , "pll3_p")
            },
            crate::pac::rcc::vals::Saisel::I2S_CKIN => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . i2s_ckin . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI2" , "i2s_ckin")
            },
            crate::pac::rcc::vals::Saisel::PER => match crate::pac::RCC.d1ccipr().read().persel() {
                crate::pac::rcc::vals::Persel::HSI => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI2" , "hsi")
                },
                crate::pac::rcc::vals::Persel::CSI => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI2" , "csi")
                },
                crate::pac::rcc::vals::Persel::HSE => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI2" , "hse")
                },
                #[allow(unreachable_patterns)]
                _ => panic!(
                    "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                    "SPI2"
                ),
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SPI2"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 14u8)),
            (58u8, 14u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI3 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip1r().read().spi123sel() {
            crate::pac::rcc::vals::Saisel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI3" , "pll1_q")
            },
            crate::pac::rcc::vals::Saisel::PLL2_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI3" , "pll2_p")
            },
            crate::pac::rcc::vals::Saisel::PLL3_P => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_p . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI3" , "pll3_p")
            },
            crate::pac::rcc::vals::Saisel::I2S_CKIN => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . i2s_ckin . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI3" , "i2s_ckin")
            },
            crate::pac::rcc::vals::Saisel::PER => match crate::pac::RCC.d1ccipr().read().persel() {
                crate::pac::rcc::vals::Persel::HSI => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI3" , "hsi")
                },
                crate::pac::rcc::vals::Persel::CSI => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI3" , "csi")
                },
                crate::pac::rcc::vals::Persel::HSE => unsafe {
                    unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI3" , "hse")
                },
                #[allow(unreachable_patterns)]
                _ => panic!(
                    "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                    "SPI3"
                ),
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SPI3"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI3" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 15u8)),
            (58u8, 15u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI4 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip1r().read().spi45sel() {
            crate::pac::rcc::vals::Spi45sel::PCLK2 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI4" , "pclk2")
            },
            crate::pac::rcc::vals::Spi45sel::PLL2_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI4" , "pll2_q")
            },
            crate::pac::rcc::vals::Spi45sel::PLL3_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI4" , "pll3_q")
            },
            crate::pac::rcc::vals::Spi45sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI4" , "hsi")
            },
            crate::pac::rcc::vals::Spi45sel::CSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI4" , "csi")
            },
            crate::pac::rcc::vals::Spi45sel::HSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI4" , "hse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SPI4"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI4" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((38u8, 13u8)),
            (60u8, 13u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI4 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SPI6 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d3ccipr().read().spi6sel() {
            crate::pac::rcc::vals::Spi6sel::PCLK4 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI6" , "pclk4")
            },
            crate::pac::rcc::vals::Spi6sel::PLL2_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI6" , "pll2_q")
            },
            crate::pac::rcc::vals::Spi6sel::PLL3_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI6" , "pll3_q")
            },
            crate::pac::rcc::vals::Spi6sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI6" , "hsi")
            },
            crate::pac::rcc::vals::Spi6sel::CSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI6" , "csi")
            },
            crate::pac::rcc::vals::Spi6sel::HSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI6" , "hse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "SPI6"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SPI6" , "pclk4")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((39u8, 5u8)),
            (61u8, 5u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SPI6 {}
impl crate::rcc::SealedRccPeripheral for peripherals::SYSCFG {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SYSCFG" , "pclk4")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk4 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "SYSCFG" , "pclk4")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((39u8, 1u8)),
            (61u8, 1u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::SYSCFG {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM1" , "pclk2_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((38u8, 0u8)),
            (60u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM12 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM12" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM12" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 6u8)),
            (58u8, 6u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM12 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM13 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM13" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM13" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 7u8)),
            (58u8, 7u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM13 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM14 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM14" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM14" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 8u8)),
            (58u8, 8u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM14 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM15 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM15" , "pclk2_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM15" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((38u8, 16u8)),
            (60u8, 16u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM15 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM16 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM16" , "pclk2_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM16" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((38u8, 17u8)),
            (60u8, 17u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM16 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM17 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM17" , "pclk2_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM17" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((38u8, 18u8)),
            (60u8, 18u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM17 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM2 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM2" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 0u8)),
            (58u8, 0u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM3 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM3" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM3" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 1u8)),
            (58u8, 1u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM4 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM4" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM4" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 2u8)),
            (58u8, 2u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM4 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM5 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM5" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM5" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 3u8)),
            (58u8, 3u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM5 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM6 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM6" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM6" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 4u8)),
            (58u8, 4u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM6 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM7 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM7" , "pclk1_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM7" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 5u8)),
            (58u8, 5u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM7 {}
impl crate::rcc::SealedRccPeripheral for peripherals::TIM8 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2_tim . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM8" , "pclk2_tim")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "TIM8" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((38u8, 1u8)),
            (60u8, 1u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::TIM8 {}
impl crate::rcc::SealedRccPeripheral for peripherals::UART4 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip2r().read().usart234578sel() {
            crate::pac::rcc::vals::Usart234578sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "pclk1")
            },
            crate::pac::rcc::vals::Usart234578sel::PLL2_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "pll2_q")
            },
            crate::pac::rcc::vals::Usart234578sel::PLL3_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "pll3_q")
            },
            crate::pac::rcc::vals::Usart234578sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "hsi")
            },
            crate::pac::rcc::vals::Usart234578sel::CSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "csi")
            },
            crate::pac::rcc::vals::Usart234578sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "UART4"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART4" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 19u8)),
            (58u8, 19u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::UART4 {}
impl crate::rcc::SealedRccPeripheral for peripherals::UART5 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip2r().read().usart234578sel() {
            crate::pac::rcc::vals::Usart234578sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "pclk1")
            },
            crate::pac::rcc::vals::Usart234578sel::PLL2_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "pll2_q")
            },
            crate::pac::rcc::vals::Usart234578sel::PLL3_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "pll3_q")
            },
            crate::pac::rcc::vals::Usart234578sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "hsi")
            },
            crate::pac::rcc::vals::Usart234578sel::CSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "csi")
            },
            crate::pac::rcc::vals::Usart234578sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "UART5"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART5" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 20u8)),
            (58u8, 20u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::UART5 {}
impl crate::rcc::SealedRccPeripheral for peripherals::UART7 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip2r().read().usart234578sel() {
            crate::pac::rcc::vals::Usart234578sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART7" , "pclk1")
            },
            crate::pac::rcc::vals::Usart234578sel::PLL2_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART7" , "pll2_q")
            },
            crate::pac::rcc::vals::Usart234578sel::PLL3_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART7" , "pll3_q")
            },
            crate::pac::rcc::vals::Usart234578sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART7" , "hsi")
            },
            crate::pac::rcc::vals::Usart234578sel::CSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART7" , "csi")
            },
            crate::pac::rcc::vals::Usart234578sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART7" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "UART7"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART7" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 30u8)),
            (58u8, 30u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::UART7 {}
impl crate::rcc::SealedRccPeripheral for peripherals::UART8 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip2r().read().usart234578sel() {
            crate::pac::rcc::vals::Usart234578sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART8" , "pclk1")
            },
            crate::pac::rcc::vals::Usart234578sel::PLL2_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART8" , "pll2_q")
            },
            crate::pac::rcc::vals::Usart234578sel::PLL3_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART8" , "pll3_q")
            },
            crate::pac::rcc::vals::Usart234578sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART8" , "hsi")
            },
            crate::pac::rcc::vals::Usart234578sel::CSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART8" , "csi")
            },
            crate::pac::rcc::vals::Usart234578sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART8" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "UART8"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "UART8" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 31u8)),
            (58u8, 31u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::UART8 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART1 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip2r().read().usart16910sel() {
            crate::pac::rcc::vals::Usart16910sel::PCLK2 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "pclk2")
            },
            crate::pac::rcc::vals::Usart16910sel::PLL2_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "pll2_q")
            },
            crate::pac::rcc::vals::Usart16910sel::PLL3_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "pll3_q")
            },
            crate::pac::rcc::vals::Usart16910sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "hsi")
            },
            crate::pac::rcc::vals::Usart16910sel::CSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "csi")
            },
            crate::pac::rcc::vals::Usart16910sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USART1"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART1" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((38u8, 4u8)),
            (60u8, 4u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART1 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART2 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip2r().read().usart234578sel() {
            crate::pac::rcc::vals::Usart234578sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "pclk1")
            },
            crate::pac::rcc::vals::Usart234578sel::PLL2_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "pll2_q")
            },
            crate::pac::rcc::vals::Usart234578sel::PLL3_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "pll3_q")
            },
            crate::pac::rcc::vals::Usart234578sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "hsi")
            },
            crate::pac::rcc::vals::Usart234578sel::CSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "csi")
            },
            crate::pac::rcc::vals::Usart234578sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USART2"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART2" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 17u8)),
            (58u8, 17u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART2 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART3 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip2r().read().usart234578sel() {
            crate::pac::rcc::vals::Usart234578sel::PCLK1 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "pclk1")
            },
            crate::pac::rcc::vals::Usart234578sel::PLL2_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "pll2_q")
            },
            crate::pac::rcc::vals::Usart234578sel::PLL3_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "pll3_q")
            },
            crate::pac::rcc::vals::Usart234578sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "hsi")
            },
            crate::pac::rcc::vals::Usart234578sel::CSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "csi")
            },
            crate::pac::rcc::vals::Usart234578sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USART3"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART3" , "pclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((36u8, 18u8)),
            (58u8, 18u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART3 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USART6 {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip2r().read().usart16910sel() {
            crate::pac::rcc::vals::Usart16910sel::PCLK2 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART6" , "pclk2")
            },
            crate::pac::rcc::vals::Usart16910sel::PLL2_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll2_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART6" , "pll2_q")
            },
            crate::pac::rcc::vals::Usart16910sel::PLL3_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART6" , "pll3_q")
            },
            crate::pac::rcc::vals::Usart16910sel::HSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART6" , "hsi")
            },
            crate::pac::rcc::vals::Usart16910sel::CSI => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . csi . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART6" , "csi")
            },
            crate::pac::rcc::vals::Usart16910sel::LSE => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . lse . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART6" , "lse")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USART6"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk2 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USART6" , "pclk2")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((38u8, 5u8)),
            (60u8, 5u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USART6 {}
impl crate::rcc::SealedRccPeripheral for peripherals::USB_OTG_FS {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip2r().read().usbsel() {
            crate::pac::rcc::vals::Usbsel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_FS" , "pll1_q")
            },
            crate::pac::rcc::vals::Usbsel::PLL3_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_FS" , "pll3_q")
            },
            crate::pac::rcc::vals::Usbsel::HSI48 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi48 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_FS" , "hsi48")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USB_OTG_FS"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_FS" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((32u8, 27u8)),
            (54u8, 27u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USB_OTG_FS {}
impl crate::rcc::SealedRccPeripheral for peripherals::USB_OTG_HS {
    fn frequency() -> crate::time::Hertz {
        match crate::pac::RCC.d2ccip2r().read().usbsel() {
            crate::pac::rcc::vals::Usbsel::PLL1_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll1_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_HS" , "pll1_q")
            },
            crate::pac::rcc::vals::Usbsel::PLL3_Q => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . pll3_q . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_HS" , "pll3_q")
            },
            crate::pac::rcc::vals::Usbsel::HSI48 => unsafe {
                unwrap ! (crate :: rcc :: get_freqs () . hsi48 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_HS" , "hsi48")
            },
            #[allow(unreachable_patterns)]
            _ => panic!(
                "attempted to use peripheral '{}' but its clock mux is not set to a valid \
                         clock. Change 'config.rcc.mux' to another clock.",
                "USB_OTG_HS"
            ),
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . hclk1 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "USB_OTG_HS" , "hclk1")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            Some((32u8, 25u8)),
            (54u8, 25u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::USB_OTG_HS {}
impl crate::rcc::SealedRccPeripheral for peripherals::WWDG1 {
    fn frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk3 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "WWDG1" , "pclk3")
        }
    }
    fn bus_frequency() -> crate::time::Hertz {
        unsafe {
            unwrap ! (crate :: rcc :: get_freqs () . pclk3 . to_hertz () , "peripheral '{}' is configured to use the '{}' clock, which is not running. \
                    Either enable it in 'config.rcc' or change 'config.rcc.mux' to use another clock" , "WWDG1" , "pclk3")
        }
    }
    const RCC_INFO: crate::rcc::RccInfo = unsafe {
        crate::rcc::RccInfo::new(
            None,
            (57u8, 6u8),
            None,
            #[cfg(feature = "low-power")]
            crate::rcc::StopMode::Stop1,
        )
    };
}
impl crate::rcc::RccPeripheral for peripherals::WWDG1 {}
pub(crate) static mut REFCOUNTS: [u8; 3usize] = [0u8, 0u8, 0u8];
pub mod mux {
    pub use crate::pac::rcc::vals::Adcsel;
    pub use crate::pac::rcc::vals::Cecsel;
    pub use crate::pac::rcc::vals::Fdcansel;
    pub use crate::pac::rcc::vals::Fmcsel;
    pub use crate::pac::rcc::vals::I2c1235sel;
    pub use crate::pac::rcc::vals::I2c4sel;
    pub use crate::pac::rcc::vals::Lptim1sel;
    pub use crate::pac::rcc::vals::Lptim2sel;
    pub use crate::pac::rcc::vals::Lpuartsel;
    pub use crate::pac::rcc::vals::Persel;
    pub use crate::pac::rcc::vals::Rngsel;
    pub use crate::pac::rcc::vals::Rtcsel;
    pub use crate::pac::rcc::vals::Saisel;
    pub use crate::pac::rcc::vals::Sdmmcsel;
    pub use crate::pac::rcc::vals::Spdifrxsel;
    pub use crate::pac::rcc::vals::Spi45sel;
    pub use crate::pac::rcc::vals::Spi6sel;
    pub use crate::pac::rcc::vals::Usart16910sel;
    pub use crate::pac::rcc::vals::Usart234578sel;
    pub use crate::pac::rcc::vals::Usbsel;
    #[derive(Clone, Copy)]
    #[non_exhaustive]
    pub struct ClockMux {
        pub rtcsel: Rtcsel,
        pub fmcsel: Fmcsel,
        pub persel: Persel,
        pub quadspisel: Fmcsel,
        pub sdmmcsel: Sdmmcsel,
        pub fdcansel: Fdcansel,
        pub sai1sel: Saisel,
        pub sai23sel: Saisel,
        pub spdifrxsel: Spdifrxsel,
        pub spi123sel: Saisel,
        pub spi45sel: Spi45sel,
        pub cecsel: Cecsel,
        pub i2c1235sel: I2c1235sel,
        pub lptim1sel: Lptim1sel,
        pub rngsel: Rngsel,
        pub usart16910sel: Usart16910sel,
        pub usart234578sel: Usart234578sel,
        pub usbsel: Usbsel,
        pub adcsel: Adcsel,
        pub i2c4sel: I2c4sel,
        pub lptim2sel: Lptim2sel,
        pub lpuart1sel: Lpuartsel,
        pub spi6sel: Spi6sel,
    }
    impl ClockMux {
        pub(crate) const fn default() -> Self {
            unsafe { ::core::mem::zeroed() }
        }
    }
    impl Default for ClockMux {
        fn default() -> Self {
            Self::default()
        }
    }
    impl ClockMux {
        pub(crate) fn init(&self) {
            crate::pac::RCC.bdcr().modify(|w| {
                w.set_rtcsel(self.rtcsel);
            });
            crate::pac::RCC.d1ccipr().modify(|w| {
                w.set_fmcsel(self.fmcsel);
                w.set_persel(self.persel);
                w.set_quadspisel(self.quadspisel);
                w.set_sdmmcsel(self.sdmmcsel);
            });
            crate::pac::RCC.d2ccip1r().modify(|w| {
                w.set_fdcansel(self.fdcansel);
                w.set_sai1sel(self.sai1sel);
                w.set_sai23sel(self.sai23sel);
                w.set_spdifrxsel(self.spdifrxsel);
                w.set_spi123sel(self.spi123sel);
                w.set_spi45sel(self.spi45sel);
            });
            crate::pac::RCC.d2ccip2r().modify(|w| {
                w.set_cecsel(self.cecsel);
                w.set_i2c1235sel(self.i2c1235sel);
                w.set_lptim1sel(self.lptim1sel);
                w.set_rngsel(self.rngsel);
                w.set_usart16910sel(self.usart16910sel);
                w.set_usart234578sel(self.usart234578sel);
                w.set_usbsel(self.usbsel);
            });
            crate::pac::RCC.d3ccipr().modify(|w| {
                w.set_adcsel(self.adcsel);
                w.set_i2c4sel(self.i2c4sel);
                w.set_lptim2sel(self.lptim2sel);
                w.set_lpuart1sel(self.lpuart1sel);
                w.set_spi6sel(self.spi6sel);
            });
        }
    }
}
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(C)]
pub struct Clocks {
    pub csi: crate::time::MaybeHertz,
    pub hclk1: crate::time::MaybeHertz,
    pub hclk2: crate::time::MaybeHertz,
    pub hclk3: crate::time::MaybeHertz,
    pub hclk4: crate::time::MaybeHertz,
    pub hse: crate::time::MaybeHertz,
    pub hsi: crate::time::MaybeHertz,
    pub hsi48: crate::time::MaybeHertz,
    pub i2s_ckin: crate::time::MaybeHertz,
    pub lse: crate::time::MaybeHertz,
    pub lsi: crate::time::MaybeHertz,
    pub pclk1: crate::time::MaybeHertz,
    pub pclk1_tim: crate::time::MaybeHertz,
    pub pclk2: crate::time::MaybeHertz,
    pub pclk2_tim: crate::time::MaybeHertz,
    pub pclk3: crate::time::MaybeHertz,
    pub pclk4: crate::time::MaybeHertz,
    pub pll1_q: crate::time::MaybeHertz,
    pub pll2_p: crate::time::MaybeHertz,
    pub pll2_q: crate::time::MaybeHertz,
    pub pll2_r: crate::time::MaybeHertz,
    pub pll3_p: crate::time::MaybeHertz,
    pub pll3_q: crate::time::MaybeHertz,
    pub pll3_r: crate::time::MaybeHertz,
    pub rtc: crate::time::MaybeHertz,
    pub sys: crate::time::MaybeHertz,
}
pub unsafe fn init_mdma() {
    crate::pac::RCC.ahb3enr().modify(|w| w.set_mdmaen(true));
}
pub unsafe fn init_dma() {
    crate::pac::RCC.ahb1enr().modify(|w| w.set_dma1en(true));
    crate::pac::RCC.ahb1enr().modify(|w| w.set_dma2en(true));
}
pub unsafe fn init_bdma() {
    crate::pac::RCC.ahb4enr().modify(|w| w.set_bdmaen(true));
}
pub unsafe fn init_dmamux() {}
pub unsafe fn init_gpdma() {}
pub unsafe fn init_gpio() {
    crate::pac::RCC.ahb4enr().modify(|w| w.set_gpioaen(true));
    crate::pac::RCC.ahb4enr().modify(|w| w.set_gpioben(true));
    crate::pac::RCC.ahb4enr().modify(|w| w.set_gpiocen(true));
    crate::pac::RCC.ahb4enr().modify(|w| w.set_gpioden(true));
    crate::pac::RCC.ahb4enr().modify(|w| w.set_gpioeen(true));
    crate::pac::RCC.ahb4enr().modify(|w| w.set_gpiofen(true));
    crate::pac::RCC.ahb4enr().modify(|w| w.set_gpiogen(true));
    crate::pac::RCC.ahb4enr().modify(|w| w.set_gpiohen(true));
    crate::pac::RCC.ahb4enr().modify(|w| w.set_gpioien(true));
    crate::pac::RCC.ahb4enr().modify(|w| w.set_gpiojen(true));
    crate::pac::RCC.ahb4enr().modify(|w| w.set_gpioken(true));
}
impl_adc_pin!(ADC1, PA0, 16u8);
impl_adc_pin!(ADC1, PA1, 17u8);
impl_adc_pin!(ADC1, PA2, 14u8);
impl_adc_pin!(ADC1, PA3, 15u8);
impl_adc_pin!(ADC1, PA4, 18u8);
impl_adc_pin!(ADC1, PA5, 19u8);
impl_adc_pin!(ADC1, PA6, 3u8);
impl_adc_pin!(ADC1, PA7, 7u8);
impl_adc_pin!(ADC1, PB0, 9u8);
impl_adc_pin!(ADC1, PB1, 5u8);
impl_adc_pin!(ADC1, PC0, 10u8);
impl_adc_pin!(ADC1, PC1, 11u8);
impl_adc_pin!(ADC1, PC4, 4u8);
impl_adc_pin!(ADC1, PC5, 8u8);
impl_adc_pair!(ADC1, PA6, PA7, 3u8);
impl_adc_pair!(ADC1, PC4, PC5, 4u8);
impl_adc_pair!(ADC1, PB1, PB0, 5u8);
impl_adc_pair!(ADC1, PC0, PC1, 10u8);
impl_adc_pair!(ADC1, PA0, PA1, 16u8);
impl_adc_pair!(ADC1, PA4, PA5, 18u8);
impl_adc_pin!(ADC2, PA2, 14u8);
impl_adc_pin!(ADC2, PA3, 15u8);
impl_adc_pin!(ADC2, PA4, 18u8);
impl_adc_pin!(ADC2, PA5, 19u8);
impl_adc_pin!(ADC2, PA6, 3u8);
impl_adc_pin!(ADC2, PA7, 7u8);
impl_adc_pin!(ADC2, PB0, 9u8);
impl_adc_pin!(ADC2, PB1, 5u8);
impl_adc_pin!(ADC2, PC0, 10u8);
impl_adc_pin!(ADC2, PC1, 11u8);
impl_adc_pin!(ADC2, PC4, 4u8);
impl_adc_pin!(ADC2, PC5, 8u8);
impl_adc_pair!(ADC2, PA6, PA7, 3u8);
impl_adc_pair!(ADC2, PC4, PC5, 4u8);
impl_adc_pair!(ADC2, PB1, PB0, 5u8);
impl_adc_pair!(ADC2, PC0, PC1, 10u8);
impl_adc_pair!(ADC2, PA4, PA5, 18u8);
impl_adc_pin!(ADC3, PC0, 10u8);
impl_adc_pin!(ADC3, PC1, 11u8);
impl_adc_pair!(ADC3, PC0, PC1, 10u8);
impl_comp_inp_pin!(COMP1, PB0, 0u8);
impl_comp_inm_pin!(COMP1, PB1, 0u8);
impl_comp_inp_pin!(COMP1, PB2, 0u8);
impl_comp_inm_pin!(COMP1, PC4, 0u8);
impl_comp_inm_pin!(COMP2, PE10, 0u8);
impl_comp_inp_pin!(COMP2, PE11, 0u8);
impl_comp_inm_pin!(COMP2, PE7, 0u8);
impl_comp_inp_pin!(COMP2, PE9, 0u8);
pin_trait_impl!(crate::dac::DacPin<Ch1>, DAC1, PA4, 0u8);
pin_trait_impl!(crate::dac::DacPin<Ch2>, DAC1, PA5, 0u8);
pin_trait_impl!(crate::dcmi::D1Pin, DCMI, PA10, 13u8);
pin_trait_impl!(crate::dcmi::HSyncPin, DCMI, PA4, 13u8);
pin_trait_impl!(crate::dcmi::PixClkPin, DCMI, PA6, 13u8);
pin_trait_impl!(crate::dcmi::D0Pin, DCMI, PA9, 13u8);
pin_trait_impl!(crate::dcmi::D10Pin, DCMI, PB5, 13u8);
pin_trait_impl!(crate::dcmi::D5Pin, DCMI, PB6, 13u8);
pin_trait_impl!(crate::dcmi::VSyncPin, DCMI, PB7, 13u8);
pin_trait_impl!(crate::dcmi::D6Pin, DCMI, PB8, 13u8);
pin_trait_impl!(crate::dcmi::D7Pin, DCMI, PB9, 13u8);
pin_trait_impl!(crate::dcmi::D8Pin, DCMI, PC10, 13u8);
pin_trait_impl!(crate::dcmi::D4Pin, DCMI, PC11, 13u8);
pin_trait_impl!(crate::dcmi::D9Pin, DCMI, PC12, 13u8);
pin_trait_impl!(crate::dcmi::D0Pin, DCMI, PC6, 13u8);
pin_trait_impl!(crate::dcmi::D1Pin, DCMI, PC7, 13u8);
pin_trait_impl!(crate::dcmi::D2Pin, DCMI, PC8, 13u8);
pin_trait_impl!(crate::dcmi::D3Pin, DCMI, PC9, 13u8);
pin_trait_impl!(crate::dcmi::D11Pin, DCMI, PD2, 13u8);
pin_trait_impl!(crate::dcmi::D5Pin, DCMI, PD3, 13u8);
pin_trait_impl!(crate::dcmi::D10Pin, DCMI, PD6, 13u8);
pin_trait_impl!(crate::dcmi::D2Pin, DCMI, PE0, 13u8);
pin_trait_impl!(crate::dcmi::D3Pin, DCMI, PE1, 13u8);
pin_trait_impl!(crate::dcmi::D4Pin, DCMI, PE4, 13u8);
pin_trait_impl!(crate::dcmi::D6Pin, DCMI, PE5, 13u8);
pin_trait_impl!(crate::dcmi::D7Pin, DCMI, PE6, 13u8);
pin_trait_impl!(
    crate::eth::RefClkPin,
    ETH,
    PA1,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::eth::RXClkPin,
    ETH,
    PA1,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::eth::MDIOPin,
    ETH_SMA,
    PA2,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::eth::CRSPin,
    ETH,
    PA7,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::eth::RXDVPin,
    ETH,
    PA7,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::eth::RXD2Pin,
    ETH,
    PB0,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::eth::RXD3Pin,
    ETH,
    PB1,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::eth::TXEnPin,
    ETH,
    PB11,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::eth::TXD0Pin,
    ETH,
    PB12,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::eth::TXD1Pin,
    ETH,
    PB13,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::eth::TXD3Pin,
    ETH,
    PB8,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::eth::MDCPin,
    ETH_SMA,
    PC1,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::eth::TXD2Pin,
    ETH,
    PC2,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::eth::TXClkPin,
    ETH,
    PC3,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::eth::RXD0Pin,
    ETH,
    PC4,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::eth::RXD1Pin,
    ETH,
    PC5,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::eth::TXD3Pin,
    ETH,
    PE2,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(crate::can::RxPin, FDCAN1, PA11, 9u8);
pin_trait_impl!(crate::can::TxPin, FDCAN1, PA12, 9u8);
pin_trait_impl!(crate::can::RxPin, FDCAN1, PB8, 9u8);
pin_trait_impl!(crate::can::TxPin, FDCAN1, PB9, 9u8);
pin_trait_impl!(crate::can::RxPin, FDCAN1, PD0, 9u8);
pin_trait_impl!(crate::can::TxPin, FDCAN1, PD1, 9u8);
pin_trait_impl!(crate::can::RxPin, FDCAN2, PB12, 9u8);
pin_trait_impl!(crate::can::TxPin, FDCAN2, PB13, 9u8);
pin_trait_impl!(crate::can::RxPin, FDCAN2, PB5, 9u8);
pin_trait_impl!(crate::can::TxPin, FDCAN2, PB6, 9u8);
pin_trait_impl!(crate::fmc::SDNWEPin, FMC, PA7, 12u8);
pin_trait_impl!(crate::fmc::SDCKE1Pin, FMC, PB5, 12u8);
pin_trait_impl!(crate::fmc::SDNE1Pin, FMC, PB6, 12u8);
pin_trait_impl!(crate::fmc::NLPin, FMC, PB7, 12u8);
pin_trait_impl!(crate::fmc::SDNWEPin, FMC, PC0, 12u8);
pin_trait_impl!(crate::fmc::SDNE0Pin, FMC, PC2, 12u8);
pin_trait_impl!(crate::fmc::SDCKE0Pin, FMC, PC3, 12u8);
pin_trait_impl!(crate::fmc::SDNE0Pin, FMC, PC4, 12u8);
pin_trait_impl!(crate::fmc::SDCKE0Pin, FMC, PC5, 12u8);
pin_trait_impl!(crate::fmc::NWaitPin, FMC, PC6, 9u8);
pin_trait_impl!(crate::fmc::NE1Pin, FMC, PC7, 9u8);
pin_trait_impl!(crate::fmc::NCEPin, FMC, PC8, 9u8);
pin_trait_impl!(crate::fmc::NE2Pin, FMC, PC8, 9u8);
pin_trait_impl!(crate::fmc::D2Pin, FMC, PD0, 12u8);
pin_trait_impl!(crate::fmc::DA2Pin, FMC, PD0, 12u8);
pin_trait_impl!(crate::fmc::D3Pin, FMC, PD1, 12u8);
pin_trait_impl!(crate::fmc::DA3Pin, FMC, PD1, 12u8);
pin_trait_impl!(crate::fmc::D15Pin, FMC, PD10, 12u8);
pin_trait_impl!(crate::fmc::DA15Pin, FMC, PD10, 12u8);
pin_trait_impl!(crate::fmc::A16Pin, FMC, PD11, 12u8);
pin_trait_impl!(crate::fmc::A17Pin, FMC, PD12, 12u8);
pin_trait_impl!(crate::fmc::A18Pin, FMC, PD13, 12u8);
pin_trait_impl!(crate::fmc::D0Pin, FMC, PD14, 12u8);
pin_trait_impl!(crate::fmc::DA0Pin, FMC, PD14, 12u8);
pin_trait_impl!(crate::fmc::D1Pin, FMC, PD15, 12u8);
pin_trait_impl!(crate::fmc::DA1Pin, FMC, PD15, 12u8);
pin_trait_impl!(crate::fmc::ClkPin, FMC, PD3, 12u8);
pin_trait_impl!(crate::fmc::NOEPin, FMC, PD4, 12u8);
pin_trait_impl!(crate::fmc::NWEPin, FMC, PD5, 12u8);
pin_trait_impl!(crate::fmc::NWaitPin, FMC, PD6, 12u8);
pin_trait_impl!(crate::fmc::NE1Pin, FMC, PD7, 12u8);
pin_trait_impl!(crate::fmc::D13Pin, FMC, PD8, 12u8);
pin_trait_impl!(crate::fmc::DA13Pin, FMC, PD8, 12u8);
pin_trait_impl!(crate::fmc::D14Pin, FMC, PD9, 12u8);
pin_trait_impl!(crate::fmc::DA14Pin, FMC, PD9, 12u8);
pin_trait_impl!(crate::fmc::NBL0Pin, FMC, PE0, 12u8);
pin_trait_impl!(crate::fmc::NBL1Pin, FMC, PE1, 12u8);
pin_trait_impl!(crate::fmc::D7Pin, FMC, PE10, 12u8);
pin_trait_impl!(crate::fmc::DA7Pin, FMC, PE10, 12u8);
pin_trait_impl!(crate::fmc::D8Pin, FMC, PE11, 12u8);
pin_trait_impl!(crate::fmc::DA8Pin, FMC, PE11, 12u8);
pin_trait_impl!(crate::fmc::D9Pin, FMC, PE12, 12u8);
pin_trait_impl!(crate::fmc::DA9Pin, FMC, PE12, 12u8);
pin_trait_impl!(crate::fmc::D10Pin, FMC, PE13, 12u8);
pin_trait_impl!(crate::fmc::DA10Pin, FMC, PE13, 12u8);
pin_trait_impl!(crate::fmc::D11Pin, FMC, PE14, 12u8);
pin_trait_impl!(crate::fmc::DA11Pin, FMC, PE14, 12u8);
pin_trait_impl!(crate::fmc::D12Pin, FMC, PE15, 12u8);
pin_trait_impl!(crate::fmc::DA12Pin, FMC, PE15, 12u8);
pin_trait_impl!(crate::fmc::A23Pin, FMC, PE2, 12u8);
pin_trait_impl!(crate::fmc::A19Pin, FMC, PE3, 12u8);
pin_trait_impl!(crate::fmc::A20Pin, FMC, PE4, 12u8);
pin_trait_impl!(crate::fmc::A21Pin, FMC, PE5, 12u8);
pin_trait_impl!(crate::fmc::A22Pin, FMC, PE6, 12u8);
pin_trait_impl!(crate::fmc::D4Pin, FMC, PE7, 12u8);
pin_trait_impl!(crate::fmc::DA4Pin, FMC, PE7, 12u8);
pin_trait_impl!(crate::fmc::D5Pin, FMC, PE8, 12u8);
pin_trait_impl!(crate::fmc::DA5Pin, FMC, PE8, 12u8);
pin_trait_impl!(crate::fmc::D6Pin, FMC, PE9, 12u8);
pin_trait_impl!(crate::fmc::DA6Pin, FMC, PE9, 12u8);
pin_trait_impl!(
    crate::i2c::SclPin,
    I2C1,
    PB6,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C1,
    PB7,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SclPin,
    I2C1,
    PB8,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C1,
    PB9,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SclPin,
    I2C2,
    PB10,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C2,
    PB11,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SclPin,
    I2C3,
    PA8,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C3,
    PC9,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SclPin,
    I2C4,
    PB6,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C4,
    PB7,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SclPin,
    I2C4,
    PB8,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C4,
    PB9,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SclPin,
    I2C4,
    PD12,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::i2c::SdaPin,
    I2C4,
    PD13,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM1, PD13, 1u8);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM2, PB13, 3u8);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM3, PA1, 3u8);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM4, PA2, 3u8);
pin_trait_impl!(crate::lptim::OutputPin, LPTIM5, PA3, 3u8);
pin_trait_impl!(
    crate::usart::RxPin,
    LPUART1,
    PA10,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    LPUART1,
    PA11,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    LPUART1,
    PA12,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    LPUART1,
    PA12,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    LPUART1,
    PA9,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    LPUART1,
    PB6,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    LPUART1,
    PB7,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(crate::ltdc::R2Pin, LTDC, PA1, 14u8);
pin_trait_impl!(crate::ltdc::B1Pin, LTDC, PA10, 14u8);
pin_trait_impl!(crate::ltdc::B4Pin, LTDC, PA10, 12u8);
pin_trait_impl!(crate::ltdc::R4Pin, LTDC, PA11, 14u8);
pin_trait_impl!(crate::ltdc::R5Pin, LTDC, PA12, 14u8);
pin_trait_impl!(crate::ltdc::R1Pin, LTDC, PA2, 14u8);
pin_trait_impl!(crate::ltdc::B2Pin, LTDC, PA3, 9u8);
pin_trait_impl!(crate::ltdc::B5Pin, LTDC, PA3, 14u8);
pin_trait_impl!(crate::ltdc::VsyncPin, LTDC, PA4, 14u8);
pin_trait_impl!(crate::ltdc::R4Pin, LTDC, PA5, 14u8);
pin_trait_impl!(crate::ltdc::G2Pin, LTDC, PA6, 14u8);
pin_trait_impl!(crate::ltdc::B3Pin, LTDC, PA8, 13u8);
pin_trait_impl!(crate::ltdc::R6Pin, LTDC, PA8, 14u8);
pin_trait_impl!(crate::ltdc::R5Pin, LTDC, PA9, 14u8);
pin_trait_impl!(crate::ltdc::G1Pin, LTDC, PB0, 14u8);
pin_trait_impl!(crate::ltdc::R3Pin, LTDC, PB0, 9u8);
pin_trait_impl!(crate::ltdc::G0Pin, LTDC, PB1, 14u8);
pin_trait_impl!(crate::ltdc::R6Pin, LTDC, PB1, 9u8);
pin_trait_impl!(crate::ltdc::G4Pin, LTDC, PB10, 14u8);
pin_trait_impl!(crate::ltdc::G5Pin, LTDC, PB11, 14u8);
pin_trait_impl!(crate::ltdc::B6Pin, LTDC, PB8, 14u8);
pin_trait_impl!(crate::ltdc::B7Pin, LTDC, PB9, 14u8);
pin_trait_impl!(crate::ltdc::R5Pin, LTDC, PC0, 14u8);
pin_trait_impl!(crate::ltdc::R2Pin, LTDC, PC10, 14u8);
pin_trait_impl!(crate::ltdc::HsyncPin, LTDC, PC6, 14u8);
pin_trait_impl!(crate::ltdc::G6Pin, LTDC, PC7, 14u8);
pin_trait_impl!(crate::ltdc::B2Pin, LTDC, PC9, 14u8);
pin_trait_impl!(crate::ltdc::G3Pin, LTDC, PC9, 10u8);
pin_trait_impl!(crate::ltdc::B3Pin, LTDC, PD10, 14u8);
pin_trait_impl!(crate::ltdc::G7Pin, LTDC, PD3, 14u8);
pin_trait_impl!(crate::ltdc::B2Pin, LTDC, PD6, 14u8);
pin_trait_impl!(crate::ltdc::G3Pin, LTDC, PE11, 14u8);
pin_trait_impl!(crate::ltdc::B4Pin, LTDC, PE12, 14u8);
pin_trait_impl!(crate::ltdc::DePin, LTDC, PE13, 14u8);
pin_trait_impl!(crate::ltdc::ClkPin, LTDC, PE14, 14u8);
pin_trait_impl!(crate::ltdc::R7Pin, LTDC, PE15, 14u8);
pin_trait_impl!(crate::ltdc::B0Pin, LTDC, PE4, 14u8);
pin_trait_impl!(crate::ltdc::G0Pin, LTDC, PE5, 14u8);
pin_trait_impl!(crate::ltdc::G1Pin, LTDC, PE6, 14u8);
impl_opamp_vn_pin!(OPAMP1, PA7, 1u8);
impl_opamp_vp_pin!(OPAMP1, PB0, 0u8);
impl_opamp_vout_pin!(OPAMP1, PC4);
impl_opamp_external_output!(OPAMP1, ADC1, 4u8);
impl_opamp_external_output!(OPAMP1, ADC2, 4u8);
impl_opamp_bias_pin!(OPAMP1, PC5, 0u8);
impl_opamp_vn_pin!(OPAMP1, PC5, 0u8);
impl_opamp_vout_pin!(OPAMP2, PE7);
impl_opamp_bias_pin!(OPAMP2, PE8, 0u8);
impl_opamp_vn_pin!(OPAMP2, PE8, 0u8);
impl_opamp_vp_pin!(OPAMP2, PE9, 0u8);
pin_trait_impl!(crate::qspi::BK1D3Pin, QUADSPI, PA1, 9u8);
pin_trait_impl!(crate::qspi::BK1NSSPin, QUADSPI, PB10, 9u8);
pin_trait_impl!(crate::qspi::SckPin, QUADSPI, PB2, 9u8);
pin_trait_impl!(crate::qspi::BK1NSSPin, QUADSPI, PB6, 10u8);
pin_trait_impl!(crate::qspi::BK1D1Pin, QUADSPI, PC10, 9u8);
pin_trait_impl!(crate::qspi::BK2NSSPin, QUADSPI, PC11, 9u8);
pin_trait_impl!(crate::qspi::BK1D0Pin, QUADSPI, PC9, 9u8);
pin_trait_impl!(crate::qspi::BK1D0Pin, QUADSPI, PD11, 9u8);
pin_trait_impl!(crate::qspi::BK1D1Pin, QUADSPI, PD12, 9u8);
pin_trait_impl!(crate::qspi::BK1D3Pin, QUADSPI, PD13, 9u8);
pin_trait_impl!(crate::qspi::BK2D3Pin, QUADSPI, PE10, 10u8);
pin_trait_impl!(crate::qspi::BK1D2Pin, QUADSPI, PE2, 9u8);
pin_trait_impl!(crate::qspi::BK2D0Pin, QUADSPI, PE7, 10u8);
pin_trait_impl!(crate::qspi::BK2D1Pin, QUADSPI, PE8, 10u8);
pin_trait_impl!(crate::qspi::BK2D2Pin, QUADSPI, PE9, 10u8);
pin_trait_impl!(crate::rcc::McoPin, MCO1, PA8, 0u8);
pin_trait_impl!(crate::rcc::McoPin, MCO2, PC9, 0u8);
pin_trait_impl!(crate::sai::SdPin<A>, SAI1, PB2, 6u8);
pin_trait_impl!(crate::sai::SdPin<A>, SAI1, PC1, 6u8);
pin_trait_impl!(crate::sai::SdPin<A>, SAI1, PD6, 6u8);
pin_trait_impl!(crate::sai::MclkPin<A>, SAI1, PE2, 6u8);
pin_trait_impl!(crate::sai::SdPin<B>, SAI1, PE3, 6u8);
pin_trait_impl!(crate::sai::FsPin<A>, SAI1, PE4, 6u8);
pin_trait_impl!(crate::sai::SckPin<A>, SAI1, PE5, 6u8);
pin_trait_impl!(crate::sai::SdPin<A>, SAI1, PE6, 6u8);
pin_trait_impl!(crate::sai::SdPin<B>, SAI2, PA0, 10u8);
pin_trait_impl!(crate::sai::MclkPin<B>, SAI2, PA1, 10u8);
pin_trait_impl!(crate::sai::FsPin<B>, SAI2, PA12, 8u8);
pin_trait_impl!(crate::sai::SckPin<B>, SAI2, PA2, 8u8);
pin_trait_impl!(crate::sai::FsPin<B>, SAI2, PC0, 8u8);
pin_trait_impl!(crate::sai::SdPin<A>, SAI2, PD11, 10u8);
pin_trait_impl!(crate::sai::FsPin<A>, SAI2, PD12, 10u8);
pin_trait_impl!(crate::sai::SckPin<A>, SAI2, PD13, 10u8);
pin_trait_impl!(crate::sai::MclkPin<A>, SAI2, PE0, 10u8);
pin_trait_impl!(crate::sai::SdPin<B>, SAI2, PE11, 10u8);
pin_trait_impl!(crate::sai::SckPin<B>, SAI2, PE12, 10u8);
pin_trait_impl!(crate::sai::FsPin<B>, SAI2, PE13, 10u8);
pin_trait_impl!(crate::sai::MclkPin<B>, SAI2, PE14, 10u8);
pin_trait_impl!(crate::sai::MclkPin<B>, SAI2, PE6, 10u8);
pin_trait_impl!(crate::sai::SckPin<A>, SAI3, PD0, 6u8);
pin_trait_impl!(crate::sai::SdPin<A>, SAI3, PD1, 6u8);
pin_trait_impl!(crate::sai::FsPin<B>, SAI3, PD10, 6u8);
pin_trait_impl!(crate::sai::MclkPin<B>, SAI3, PD14, 6u8);
pin_trait_impl!(crate::sai::MclkPin<A>, SAI3, PD15, 6u8);
pin_trait_impl!(crate::sai::FsPin<A>, SAI3, PD4, 6u8);
pin_trait_impl!(crate::sai::SckPin<B>, SAI3, PD8, 6u8);
pin_trait_impl!(crate::sai::SdPin<B>, SAI3, PD9, 6u8);
pin_trait_impl!(crate::sai::SdPin<A>, SAI4, PB2, 8u8);
pin_trait_impl!(crate::sai::SdPin<A>, SAI4, PC1, 8u8);
pin_trait_impl!(crate::sai::SdPin<A>, SAI4, PD6, 8u8);
pin_trait_impl!(crate::sai::MclkPin<A>, SAI4, PE2, 8u8);
pin_trait_impl!(crate::sai::SdPin<B>, SAI4, PE3, 8u8);
pin_trait_impl!(crate::sai::FsPin<A>, SAI4, PE4, 8u8);
pin_trait_impl!(crate::sai::SckPin<A>, SAI4, PE5, 8u8);
pin_trait_impl!(crate::sai::SdPin<A>, SAI4, PE6, 8u8);
pin_trait_impl!(crate::sdmmc::D4Pin, SDMMC1, PB8, 12u8);
pin_trait_impl!(crate::sdmmc::D5Pin, SDMMC1, PB9, 12u8);
pin_trait_impl!(crate::sdmmc::D2Pin, SDMMC1, PC10, 12u8);
pin_trait_impl!(crate::sdmmc::D3Pin, SDMMC1, PC11, 12u8);
pin_trait_impl!(crate::sdmmc::CkPin, SDMMC1, PC12, 12u8);
pin_trait_impl!(crate::sdmmc::D6Pin, SDMMC1, PC6, 12u8);
pin_trait_impl!(crate::sdmmc::D7Pin, SDMMC1, PC7, 12u8);
pin_trait_impl!(crate::sdmmc::D0Pin, SDMMC1, PC8, 12u8);
pin_trait_impl!(crate::sdmmc::D1Pin, SDMMC1, PC9, 12u8);
pin_trait_impl!(crate::sdmmc::CmdPin, SDMMC1, PD2, 12u8);
pin_trait_impl!(crate::sdmmc::CmdPin, SDMMC2, PA0, 9u8);
pin_trait_impl!(crate::sdmmc::D0Pin, SDMMC2, PB14, 9u8);
pin_trait_impl!(crate::sdmmc::D1Pin, SDMMC2, PB15, 9u8);
pin_trait_impl!(crate::sdmmc::D2Pin, SDMMC2, PB3, 9u8);
pin_trait_impl!(crate::sdmmc::D3Pin, SDMMC2, PB4, 9u8);
pin_trait_impl!(crate::sdmmc::D4Pin, SDMMC2, PB8, 10u8);
pin_trait_impl!(crate::sdmmc::D5Pin, SDMMC2, PB9, 10u8);
pin_trait_impl!(crate::sdmmc::CkPin, SDMMC2, PC1, 9u8);
pin_trait_impl!(crate::sdmmc::D6Pin, SDMMC2, PC6, 10u8);
pin_trait_impl!(crate::sdmmc::D7Pin, SDMMC2, PC7, 10u8);
pin_trait_impl!(crate::sdmmc::CkPin, SDMMC2, PD6, 11u8);
pin_trait_impl!(crate::sdmmc::CmdPin, SDMMC2, PD7, 11u8);
impl_spdifrx_pin!(SPDIFRX1, PC4, 9u8, 2u8);
impl_spdifrx_pin!(SPDIFRX1, PC5, 9u8, 3u8);
impl_spdifrx_pin!(SPDIFRX1, PD7, 9u8, 0u8);
impl_spdifrx_pin!(SPDIFRX1, PD8, 9u8, 1u8);
pin_trait_impl!(
    crate::spi::WsPin,
    SPI1,
    PA15,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI1,
    PA15,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::WsPin,
    SPI1,
    PA4,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI1,
    PA4,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CkPin,
    SPI1,
    PA5,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI1,
    PA5,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI1,
    PA6,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI1,
    PA6,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI1,
    PA7,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI1,
    PA7,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CkPin,
    SPI1,
    PB3,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI1,
    PB3,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI1,
    PB4,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI1,
    PB4,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI1,
    PB5,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI1,
    PB5,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MckPin,
    SPI1,
    PC4,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI1,
    PD7,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI1,
    PD7,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::WsPin,
    SPI2,
    PA11,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI2,
    PA11,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CkPin,
    SPI2,
    PA12,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI2,
    PA12,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CkPin,
    SPI2,
    PA9,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI2,
    PA9,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CkPin,
    SPI2,
    PB10,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI2,
    PB10,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::WsPin,
    SPI2,
    PB12,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI2,
    PB12,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CkPin,
    SPI2,
    PB13,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI2,
    PB13,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI2,
    PB14,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI2,
    PB14,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI2,
    PB15,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI2,
    PB15,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::WsPin,
    SPI2,
    PB4,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI2,
    PB4,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::WsPin,
    SPI2,
    PB9,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI2,
    PB9,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI2,
    PC1,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI2,
    PC1,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI2,
    PC2,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI2,
    PC2,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI2,
    PC3,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI2,
    PC3,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MckPin,
    SPI2,
    PC6,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CkPin,
    SPI2,
    PD3,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI2,
    PD3,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::WsPin,
    SPI3,
    PA15,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI3,
    PA15,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::WsPin,
    SPI3,
    PA4,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI3,
    PA4,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI3,
    PB2,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI3,
    PB2,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CkPin,
    SPI3,
    PB3,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI3,
    PB3,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI3,
    PB4,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI3,
    PB4,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI3,
    PB5,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI3,
    PB5,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CkPin,
    SPI3,
    PC10,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI3,
    PC10,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI3,
    PC11,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI3,
    PC11,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI3,
    PC12,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI3,
    PC12,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MckPin,
    SPI3,
    PC7,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::I2sSdPin,
    SPI3,
    PD6,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI3,
    PD6,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI4,
    PE11,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI4,
    PE12,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI4,
    PE13,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI4,
    PE14,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI4,
    PE2,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI4,
    PE4,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI4,
    PE5,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI4,
    PE6,
    5u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI6,
    PA15,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::CsPin,
    SPI6,
    PA4,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI6,
    PA5,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI6,
    PA6,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI6,
    PA7,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::SckPin,
    SPI6,
    PB3,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MisoPin,
    SPI6,
    PB4,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::spi::MosiPin,
    SPI6,
    PB5,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM1,
    PA10,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM1,
    PA11,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM1,
    PA12,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM1,
    PA6,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator1Pin<BkIn1>,
    TIM1,
    PA6,
    12u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator2Pin<BkIn1>,
    TIM1,
    PA6,
    12u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM1,
    PA7,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM1,
    PA8,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM1,
    PA9,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch2>,
    TIM1,
    PB0,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch3>,
    TIM1,
    PB1,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM1,
    PB12,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator1Pin<BkIn1>,
    TIM1,
    PB12,
    13u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator2Pin<BkIn1>,
    TIM1,
    PB12,
    13u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM1,
    PB13,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch2>,
    TIM1,
    PB14,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch3>,
    TIM1,
    PB15,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch2>,
    TIM1,
    PE10,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM1,
    PE11,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch3>,
    TIM1,
    PE12,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM1,
    PE13,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM1,
    PE14,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM1,
    PE15,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator1Pin<BkIn1>,
    TIM1,
    PE15,
    13u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator2Pin<BkIn1>,
    TIM1,
    PE15,
    13u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn2>,
    TIM1,
    PE6,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator1Pin<BkIn2>,
    TIM1,
    PE6,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator2Pin<BkIn2>,
    TIM1,
    PE6,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM1,
    PE7,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM1,
    PE8,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM1,
    PE9,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM12,
    PB14,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM12,
    PB15,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM13,
    PA6,
    9u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM14,
    PA7,
    9u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM15,
    PA0,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM15,
    PA1,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM15,
    PA2,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM15,
    PA3,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM15,
    PE3,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM15,
    PE4,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM15,
    PE5,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM15,
    PE6,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM16,
    PB4,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM16,
    PB6,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM16,
    PB8,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM17,
    PB5,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM17,
    PB7,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM17,
    PB9,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM2,
    PA0,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM2,
    PA0,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM2,
    PA1,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM2,
    PA15,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM2,
    PA15,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM2,
    PA2,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM2,
    PA3,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM2,
    PA5,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM2,
    PA5,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM2,
    PB10,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM2,
    PB11,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM2,
    PB3,
    1u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM3,
    PA6,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM3,
    PA7,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM3,
    PB0,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM3,
    PB1,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM3,
    PB4,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM3,
    PB5,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM3,
    PC6,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM3,
    PC7,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM3,
    PC8,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM3,
    PC9,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM3,
    PD2,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM4,
    PB6,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM4,
    PB7,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM4,
    PB8,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM4,
    PB9,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM4,
    PD12,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM4,
    PD13,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM4,
    PD14,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM4,
    PD15,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM4,
    PE0,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM5,
    PA0,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM5,
    PA1,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM5,
    PA2,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM5,
    PA3,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM5,
    PA4,
    2u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::ExternalTriggerPin,
    TIM8,
    PA0,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM8,
    PA5,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn1>,
    TIM8,
    PA6,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator1Pin<BkIn1>,
    TIM8,
    PA6,
    10u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator2Pin<BkIn1>,
    TIM8,
    PA6,
    10u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch1>,
    TIM8,
    PA7,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputPin<BkIn2>,
    TIM8,
    PA8,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator1Pin<BkIn2>,
    TIM8,
    PA8,
    12u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::BreakInputComparator2Pin<BkIn2>,
    TIM8,
    PA8,
    12u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch2>,
    TIM8,
    PB0,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch3>,
    TIM8,
    PB1,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch2>,
    TIM8,
    PB14,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerComplementaryPin<Ch3>,
    TIM8,
    PB15,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch1>,
    TIM8,
    PC6,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch2>,
    TIM8,
    PC7,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch3>,
    TIM8,
    PC8,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::timer::TimerPin<Ch4>,
    TIM8,
    PC9,
    3u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    UART4,
    PA0,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    UART4,
    PA1,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    UART4,
    PA11,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    UART4,
    PA12,
    6u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    UART4,
    PA15,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    UART4,
    PA15,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    UART4,
    PB0,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    UART4,
    PB14,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    UART4,
    PB14,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    UART4,
    PB15,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    UART4,
    PB8,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    UART4,
    PB9,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    UART4,
    PC10,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    UART4,
    PC11,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    UART4,
    PD0,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    UART4,
    PD1,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    UART5,
    PB12,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    UART5,
    PB13,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    UART5,
    PB5,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    UART5,
    PB6,
    14u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    UART5,
    PC12,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    UART5,
    PC8,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    UART5,
    PC8,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    UART5,
    PC9,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    UART5,
    PD2,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    UART7,
    PA15,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    UART7,
    PA8,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    UART7,
    PB3,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    UART7,
    PB4,
    11u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    UART7,
    PE10,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    UART7,
    PE7,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    UART7,
    PE8,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    UART7,
    PE9,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    UART7,
    PE9,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    UART8,
    PD14,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    UART8,
    PD15,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    UART8,
    PD15,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    UART8,
    PE0,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    UART8,
    PE1,
    8u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART1,
    PA10,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    USART1,
    PA11,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    USART1,
    PA12,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    USART1,
    PA12,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART1,
    PA8,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART1,
    PA9,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART1,
    PB14,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART1,
    PB15,
    4u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART1,
    PB6,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART1,
    PB7,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    USART2,
    PA0,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    USART2,
    PA1,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    USART2,
    PA1,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART2,
    PA2,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART2,
    PA3,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART2,
    PA4,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    USART2,
    PD3,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    USART2,
    PD4,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    USART2,
    PD4,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART2,
    PD5,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART2,
    PD6,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART2,
    PD7,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART3,
    PB10,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART3,
    PB11,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART3,
    PB12,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    USART3,
    PB13,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    USART3,
    PB14,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    USART3,
    PB14,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART3,
    PC10,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART3,
    PC11,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART3,
    PC12,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART3,
    PD10,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CtsPin,
    USART3,
    PD11,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::DePin,
    USART3,
    PD12,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RtsPin,
    USART3,
    PD12,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART3,
    PD8,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART3,
    PD9,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::TxPin,
    USART6,
    PC6,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::RxPin,
    USART6,
    PC7,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(
    crate::usart::CkPin,
    USART6,
    PC8,
    7u8,
    crate::gpio::AfioRemapNotApplicable
);
pin_trait_impl!(crate::usb::DmPin, USB_OTG_FS, PA11, 10u8);
pin_trait_impl!(crate::usb::DpPin, USB_OTG_FS, PA12, 10u8);
pin_trait_impl!(crate::usb::UlpiD0Pin, USB_OTG_HS, PA3, 10u8);
pin_trait_impl!(crate::usb::UlpiClkPin, USB_OTG_HS, PA5, 10u8);
pin_trait_impl!(crate::usb::UlpiD1Pin, USB_OTG_HS, PB0, 10u8);
pin_trait_impl!(crate::usb::UlpiD2Pin, USB_OTG_HS, PB1, 10u8);
pin_trait_impl!(crate::usb::UlpiD3Pin, USB_OTG_HS, PB10, 10u8);
pin_trait_impl!(crate::usb::UlpiD4Pin, USB_OTG_HS, PB11, 10u8);
pin_trait_impl!(crate::usb::UlpiD5Pin, USB_OTG_HS, PB12, 10u8);
pin_trait_impl!(crate::usb::UlpiD6Pin, USB_OTG_HS, PB13, 10u8);
pin_trait_impl!(crate::usb::DmPin, USB_OTG_HS, PB14, 12u8);
pin_trait_impl!(crate::usb::DpPin, USB_OTG_HS, PB15, 12u8);
pin_trait_impl!(crate::usb::UlpiD7Pin, USB_OTG_HS, PB5, 10u8);
pin_trait_impl!(crate::usb::UlpiStpPin, USB_OTG_HS, PC0, 10u8);
pin_trait_impl!(crate::usb::UlpiDirPin, USB_OTG_HS, PC2, 10u8);
pin_trait_impl!(crate::usb::UlpiNxtPin, USB_OTG_HS, PC3, 10u8);
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA1_CH0, 9u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA1_CH1, 9u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA1_CH2, 9u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA1_CH3, 9u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA1_CH4, 9u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA1_CH5, 9u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA1_CH6, 9u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA1_CH7, 9u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA2_CH0, 9u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA2_CH1, 9u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA2_CH2, 9u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA2_CH3, 9u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA2_CH4, 9u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA2_CH5, 9u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA2_CH6, 9u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC1, DMA2_CH7, 9u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA1_CH0, 10u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA1_CH1, 10u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA1_CH2, 10u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA1_CH3, 10u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA1_CH4, 10u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA1_CH5, 10u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA1_CH6, 10u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA1_CH7, 10u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA2_CH0, 10u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA2_CH1, 10u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA2_CH2, 10u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA2_CH3, 10u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA2_CH4, 10u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA2_CH5, 10u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA2_CH6, 10u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC2, DMA2_CH7, 10u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA1_CH0, 115u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA1_CH1, 115u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA1_CH2, 115u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA1_CH3, 115u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA1_CH4, 115u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA1_CH5, 115u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA1_CH6, 115u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA1_CH7, 115u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA2_CH0, 115u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA2_CH1, 115u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA2_CH2, 115u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA2_CH3, 115u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA2_CH4, 115u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA2_CH5, 115u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA2_CH6, 115u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, DMA2_CH7, 115u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, BDMA_CH0, 17u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, BDMA_CH1, 17u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, BDMA_CH2, 17u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, BDMA_CH3, 17u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, BDMA_CH4, 17u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, BDMA_CH5, 17u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, BDMA_CH6, 17u8, {});
dma_trait_impl!(crate::adc::RxDma, ADC3, BDMA_CH7, 17u8, {});
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, TIM1_TRGO, 1u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, TIM2_TRGO, 2u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, TIM4_TRGO, 3u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, TIM5_TRGO, 4u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, TIM6_TRGO, 5u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, TIM7_TRGO, 6u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, TIM8_TRGO, 7u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, TIM15_TRGO, 8u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, HRTIM_DAC_TRG1, 9u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, HRTIM_DAC_TRG2, 10u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, LPTIM1_TRGO, 11u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, LPTIM2_TRGO, 12u8);
trigger_trait_impl!(crate::dac::ChannelTrigger, DAC1, EXTI9_TRG, 13u8);
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA1_CH0, 67u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA1_CH1, 67u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA1_CH2, 67u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA1_CH3, 67u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA1_CH4, 67u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA1_CH5, 67u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA1_CH6, 67u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA1_CH7, 67u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA2_CH0, 67u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA2_CH1, 67u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA2_CH2, 67u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA2_CH3, 67u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA2_CH4, 67u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA2_CH5, 67u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA2_CH6, 67u8, {});
dma_trait_impl!(crate::dac::Dma<Ch1>, DAC1, DMA2_CH7, 67u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA1_CH0, 68u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA1_CH1, 68u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA1_CH2, 68u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA1_CH3, 68u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA1_CH4, 68u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA1_CH5, 68u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA1_CH6, 68u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA1_CH7, 68u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA2_CH0, 68u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA2_CH1, 68u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA2_CH2, 68u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA2_CH3, 68u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA2_CH4, 68u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA2_CH5, 68u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA2_CH6, 68u8, {});
dma_trait_impl!(crate::dac::Dma<Ch2>, DAC1, DMA2_CH7, 68u8, {});
dma_trait_impl!(crate::dcmi::FrameDma, DCMI, DMA1_CH0, 75u8, {});
dma_trait_impl!(crate::dcmi::FrameDma, DCMI, DMA1_CH1, 75u8, {});
dma_trait_impl!(crate::dcmi::FrameDma, DCMI, DMA1_CH2, 75u8, {});
dma_trait_impl!(crate::dcmi::FrameDma, DCMI, DMA1_CH3, 75u8, {});
dma_trait_impl!(crate::dcmi::FrameDma, DCMI, DMA1_CH4, 75u8, {});
dma_trait_impl!(crate::dcmi::FrameDma, DCMI, DMA1_CH5, 75u8, {});
dma_trait_impl!(crate::dcmi::FrameDma, DCMI, DMA1_CH6, 75u8, {});
dma_trait_impl!(crate::dcmi::FrameDma, DCMI, DMA1_CH7, 75u8, {});
dma_trait_impl!(crate::dcmi::FrameDma, DCMI, DMA2_CH0, 75u8, {});
dma_trait_impl!(crate::dcmi::FrameDma, DCMI, DMA2_CH1, 75u8, {});
dma_trait_impl!(crate::dcmi::FrameDma, DCMI, DMA2_CH2, 75u8, {});
dma_trait_impl!(crate::dcmi::FrameDma, DCMI, DMA2_CH3, 75u8, {});
dma_trait_impl!(crate::dcmi::FrameDma, DCMI, DMA2_CH4, 75u8, {});
dma_trait_impl!(crate::dcmi::FrameDma, DCMI, DMA2_CH5, 75u8, {});
dma_trait_impl!(crate::dcmi::FrameDma, DCMI, DMA2_CH6, 75u8, {});
dma_trait_impl!(crate::dcmi::FrameDma, DCMI, DMA2_CH7, 75u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH0, 33u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH1, 33u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH2, 33u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH3, 33u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH4, 33u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH5, 33u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH6, 33u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA1_CH7, 33u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA2_CH0, 33u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA2_CH1, 33u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA2_CH2, 33u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA2_CH3, 33u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA2_CH4, 33u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA2_CH5, 33u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA2_CH6, 33u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C1, DMA2_CH7, 33u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH0, 34u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH1, 34u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH2, 34u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH3, 34u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH4, 34u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH5, 34u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH6, 34u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA1_CH7, 34u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA2_CH0, 34u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA2_CH1, 34u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA2_CH2, 34u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA2_CH3, 34u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA2_CH4, 34u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA2_CH5, 34u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA2_CH6, 34u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C1, DMA2_CH7, 34u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH0, 35u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH1, 35u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH2, 35u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH3, 35u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH4, 35u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH5, 35u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH6, 35u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA1_CH7, 35u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA2_CH0, 35u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA2_CH1, 35u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA2_CH2, 35u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA2_CH3, 35u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA2_CH4, 35u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA2_CH5, 35u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA2_CH6, 35u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C2, DMA2_CH7, 35u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA1_CH0, 36u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA1_CH1, 36u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA1_CH2, 36u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA1_CH3, 36u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA1_CH4, 36u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA1_CH5, 36u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA1_CH6, 36u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA1_CH7, 36u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA2_CH0, 36u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA2_CH1, 36u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA2_CH2, 36u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA2_CH3, 36u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA2_CH4, 36u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA2_CH5, 36u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA2_CH6, 36u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C2, DMA2_CH7, 36u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA1_CH0, 73u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA1_CH1, 73u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA1_CH2, 73u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA1_CH3, 73u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA1_CH4, 73u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA1_CH5, 73u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA1_CH6, 73u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA1_CH7, 73u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA2_CH0, 73u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA2_CH1, 73u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA2_CH2, 73u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA2_CH3, 73u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA2_CH4, 73u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA2_CH5, 73u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA2_CH6, 73u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C3, DMA2_CH7, 73u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA1_CH0, 74u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA1_CH1, 74u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA1_CH2, 74u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA1_CH3, 74u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA1_CH4, 74u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA1_CH5, 74u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA1_CH6, 74u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA1_CH7, 74u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA2_CH0, 74u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA2_CH1, 74u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA2_CH2, 74u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA2_CH3, 74u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA2_CH4, 74u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA2_CH5, 74u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA2_CH6, 74u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C3, DMA2_CH7, 74u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C4, BDMA_CH0, 13u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C4, BDMA_CH1, 13u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C4, BDMA_CH2, 13u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C4, BDMA_CH3, 13u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C4, BDMA_CH4, 13u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C4, BDMA_CH5, 13u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C4, BDMA_CH6, 13u8, {});
dma_trait_impl!(crate::i2c::RxDma, I2C4, BDMA_CH7, 13u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C4, BDMA_CH0, 14u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C4, BDMA_CH1, 14u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C4, BDMA_CH2, 14u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C4, BDMA_CH3, 14u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C4, BDMA_CH4, 14u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C4, BDMA_CH5, 14u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C4, BDMA_CH6, 14u8, {});
dma_trait_impl!(crate::i2c::TxDma, I2C4, BDMA_CH7, 14u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART1, BDMA_CH0, 9u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART1, BDMA_CH1, 9u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART1, BDMA_CH2, 9u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART1, BDMA_CH3, 9u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART1, BDMA_CH4, 9u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART1, BDMA_CH5, 9u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART1, BDMA_CH6, 9u8, {});
dma_trait_impl!(crate::usart::RxDma, LPUART1, BDMA_CH7, 9u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART1, BDMA_CH0, 10u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART1, BDMA_CH1, 10u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART1, BDMA_CH2, 10u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART1, BDMA_CH3, 10u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART1, BDMA_CH4, 10u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART1, BDMA_CH5, 10u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART1, BDMA_CH6, 10u8, {});
dma_trait_impl!(crate::usart::TxDma, LPUART1, BDMA_CH7, 10u8, {});
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, MDMA_CH0, 22u8, {});
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, MDMA_CH1, 22u8, {});
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, MDMA_CH2, 22u8, {});
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, MDMA_CH3, 22u8, {});
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, MDMA_CH4, 22u8, {});
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, MDMA_CH5, 22u8, {});
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, MDMA_CH6, 22u8, {});
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, MDMA_CH7, 22u8, {});
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, MDMA_CH8, 22u8, {});
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, MDMA_CH9, 22u8, {});
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, MDMA_CH10, 22u8, {});
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, MDMA_CH11, 22u8, {});
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, MDMA_CH12, 22u8, {});
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, MDMA_CH13, 22u8, {});
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, MDMA_CH14, 22u8, {});
dma_trait_impl!(crate::qspi::QuadDma, QUADSPI, MDMA_CH15, 22u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA1_CH0, 87u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA1_CH1, 87u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA1_CH2, 87u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA1_CH3, 87u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA1_CH4, 87u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA1_CH5, 87u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA1_CH6, 87u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA1_CH7, 87u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA2_CH0, 87u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA2_CH1, 87u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA2_CH2, 87u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA2_CH3, 87u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA2_CH4, 87u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA2_CH5, 87u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA2_CH6, 87u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI1, DMA2_CH7, 87u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA1_CH0, 88u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA1_CH1, 88u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA1_CH2, 88u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA1_CH3, 88u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA1_CH4, 88u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA1_CH5, 88u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA1_CH6, 88u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA1_CH7, 88u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA2_CH0, 88u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA2_CH1, 88u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA2_CH2, 88u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA2_CH3, 88u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA2_CH4, 88u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA2_CH5, 88u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA2_CH6, 88u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI1, DMA2_CH7, 88u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI2, DMA1_CH0, 89u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI2, DMA1_CH1, 89u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI2, DMA1_CH2, 89u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI2, DMA1_CH3, 89u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI2, DMA1_CH4, 89u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI2, DMA1_CH5, 89u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI2, DMA1_CH6, 89u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI2, DMA1_CH7, 89u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI2, DMA2_CH0, 89u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI2, DMA2_CH1, 89u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI2, DMA2_CH2, 89u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI2, DMA2_CH3, 89u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI2, DMA2_CH4, 89u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI2, DMA2_CH5, 89u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI2, DMA2_CH6, 89u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI2, DMA2_CH7, 89u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA1_CH0, 90u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA1_CH1, 90u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA1_CH2, 90u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA1_CH3, 90u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA1_CH4, 90u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA1_CH5, 90u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA1_CH6, 90u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA1_CH7, 90u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA2_CH0, 90u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA2_CH1, 90u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA2_CH2, 90u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA2_CH3, 90u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA2_CH4, 90u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA2_CH5, 90u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA2_CH6, 90u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI2, DMA2_CH7, 90u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI3, DMA1_CH0, 113u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI3, DMA1_CH1, 113u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI3, DMA1_CH2, 113u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI3, DMA1_CH3, 113u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI3, DMA1_CH4, 113u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI3, DMA1_CH5, 113u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI3, DMA1_CH6, 113u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI3, DMA1_CH7, 113u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI3, DMA2_CH0, 113u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI3, DMA2_CH1, 113u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI3, DMA2_CH2, 113u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI3, DMA2_CH3, 113u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI3, DMA2_CH4, 113u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI3, DMA2_CH5, 113u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI3, DMA2_CH6, 113u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI3, DMA2_CH7, 113u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI3, DMA1_CH0, 114u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI3, DMA1_CH1, 114u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI3, DMA1_CH2, 114u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI3, DMA1_CH3, 114u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI3, DMA1_CH4, 114u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI3, DMA1_CH5, 114u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI3, DMA1_CH6, 114u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI3, DMA1_CH7, 114u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI3, DMA2_CH0, 114u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI3, DMA2_CH1, 114u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI3, DMA2_CH2, 114u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI3, DMA2_CH3, 114u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI3, DMA2_CH4, 114u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI3, DMA2_CH5, 114u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI3, DMA2_CH6, 114u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI3, DMA2_CH7, 114u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI4, BDMA_CH0, 15u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI4, BDMA_CH1, 15u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI4, BDMA_CH2, 15u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI4, BDMA_CH3, 15u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI4, BDMA_CH4, 15u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI4, BDMA_CH5, 15u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI4, BDMA_CH6, 15u8, {});
dma_trait_impl!(crate::sai::Dma<A>, SAI4, BDMA_CH7, 15u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI4, BDMA_CH0, 16u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI4, BDMA_CH1, 16u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI4, BDMA_CH2, 16u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI4, BDMA_CH3, 16u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI4, BDMA_CH4, 16u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI4, BDMA_CH5, 16u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI4, BDMA_CH6, 16u8, {});
dma_trait_impl!(crate::sai::Dma<B>, SAI4, BDMA_CH7, 16u8, {});
dma_trait_impl!(crate::spdifrx::Dma, SPDIFRX1, DMA1_CH0, 93u8, {});
dma_trait_impl!(crate::spdifrx::Dma, SPDIFRX1, DMA1_CH1, 93u8, {});
dma_trait_impl!(crate::spdifrx::Dma, SPDIFRX1, DMA1_CH2, 93u8, {});
dma_trait_impl!(crate::spdifrx::Dma, SPDIFRX1, DMA1_CH3, 93u8, {});
dma_trait_impl!(crate::spdifrx::Dma, SPDIFRX1, DMA1_CH4, 93u8, {});
dma_trait_impl!(crate::spdifrx::Dma, SPDIFRX1, DMA1_CH5, 93u8, {});
dma_trait_impl!(crate::spdifrx::Dma, SPDIFRX1, DMA1_CH6, 93u8, {});
dma_trait_impl!(crate::spdifrx::Dma, SPDIFRX1, DMA1_CH7, 93u8, {});
dma_trait_impl!(crate::spdifrx::Dma, SPDIFRX1, DMA2_CH0, 93u8, {});
dma_trait_impl!(crate::spdifrx::Dma, SPDIFRX1, DMA2_CH1, 93u8, {});
dma_trait_impl!(crate::spdifrx::Dma, SPDIFRX1, DMA2_CH2, 93u8, {});
dma_trait_impl!(crate::spdifrx::Dma, SPDIFRX1, DMA2_CH3, 93u8, {});
dma_trait_impl!(crate::spdifrx::Dma, SPDIFRX1, DMA2_CH4, 93u8, {});
dma_trait_impl!(crate::spdifrx::Dma, SPDIFRX1, DMA2_CH5, 93u8, {});
dma_trait_impl!(crate::spdifrx::Dma, SPDIFRX1, DMA2_CH6, 93u8, {});
dma_trait_impl!(crate::spdifrx::Dma, SPDIFRX1, DMA2_CH7, 93u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA1_CH0, 37u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA1_CH1, 37u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA1_CH2, 37u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA1_CH3, 37u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA1_CH4, 37u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA1_CH5, 37u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA1_CH6, 37u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA1_CH7, 37u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA2_CH0, 37u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA2_CH1, 37u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA2_CH2, 37u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA2_CH3, 37u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA2_CH4, 37u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA2_CH5, 37u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA2_CH6, 37u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI1, DMA2_CH7, 37u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA1_CH0, 38u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA1_CH1, 38u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA1_CH2, 38u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA1_CH3, 38u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA1_CH4, 38u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA1_CH5, 38u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA1_CH6, 38u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA1_CH7, 38u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA2_CH0, 38u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA2_CH1, 38u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA2_CH2, 38u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA2_CH3, 38u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA2_CH4, 38u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA2_CH5, 38u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA2_CH6, 38u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI1, DMA2_CH7, 38u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA1_CH0, 39u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA1_CH1, 39u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA1_CH2, 39u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA1_CH3, 39u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA1_CH4, 39u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA1_CH5, 39u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA1_CH6, 39u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA1_CH7, 39u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA2_CH0, 39u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA2_CH1, 39u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA2_CH2, 39u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA2_CH3, 39u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA2_CH4, 39u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA2_CH5, 39u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA2_CH6, 39u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI2, DMA2_CH7, 39u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA1_CH0, 40u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA1_CH1, 40u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA1_CH2, 40u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA1_CH3, 40u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA1_CH4, 40u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA1_CH5, 40u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA1_CH6, 40u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA1_CH7, 40u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA2_CH0, 40u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA2_CH1, 40u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA2_CH2, 40u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA2_CH3, 40u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA2_CH4, 40u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA2_CH5, 40u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA2_CH6, 40u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI2, DMA2_CH7, 40u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA1_CH0, 61u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA1_CH1, 61u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA1_CH2, 61u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA1_CH3, 61u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA1_CH4, 61u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA1_CH5, 61u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA1_CH6, 61u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA1_CH7, 61u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA2_CH0, 61u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA2_CH1, 61u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA2_CH2, 61u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA2_CH3, 61u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA2_CH4, 61u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA2_CH5, 61u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA2_CH6, 61u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI3, DMA2_CH7, 61u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA1_CH0, 62u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA1_CH1, 62u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA1_CH2, 62u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA1_CH3, 62u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA1_CH4, 62u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA1_CH5, 62u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA1_CH6, 62u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA1_CH7, 62u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA2_CH0, 62u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA2_CH1, 62u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA2_CH2, 62u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA2_CH3, 62u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA2_CH4, 62u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA2_CH5, 62u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA2_CH6, 62u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI3, DMA2_CH7, 62u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI4, DMA1_CH0, 83u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI4, DMA1_CH1, 83u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI4, DMA1_CH2, 83u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI4, DMA1_CH3, 83u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI4, DMA1_CH4, 83u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI4, DMA1_CH5, 83u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI4, DMA1_CH6, 83u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI4, DMA1_CH7, 83u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI4, DMA2_CH0, 83u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI4, DMA2_CH1, 83u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI4, DMA2_CH2, 83u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI4, DMA2_CH3, 83u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI4, DMA2_CH4, 83u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI4, DMA2_CH5, 83u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI4, DMA2_CH6, 83u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI4, DMA2_CH7, 83u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI4, DMA1_CH0, 84u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI4, DMA1_CH1, 84u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI4, DMA1_CH2, 84u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI4, DMA1_CH3, 84u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI4, DMA1_CH4, 84u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI4, DMA1_CH5, 84u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI4, DMA1_CH6, 84u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI4, DMA1_CH7, 84u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI4, DMA2_CH0, 84u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI4, DMA2_CH1, 84u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI4, DMA2_CH2, 84u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI4, DMA2_CH3, 84u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI4, DMA2_CH4, 84u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI4, DMA2_CH5, 84u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI4, DMA2_CH6, 84u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI4, DMA2_CH7, 84u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI6, BDMA_CH0, 11u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI6, BDMA_CH1, 11u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI6, BDMA_CH2, 11u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI6, BDMA_CH3, 11u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI6, BDMA_CH4, 11u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI6, BDMA_CH5, 11u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI6, BDMA_CH6, 11u8, {});
dma_trait_impl!(crate::spi::RxDma, SPI6, BDMA_CH7, 11u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI6, BDMA_CH0, 12u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI6, BDMA_CH1, 12u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI6, BDMA_CH2, 12u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI6, BDMA_CH3, 12u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI6, BDMA_CH4, 12u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI6, BDMA_CH5, 12u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI6, BDMA_CH6, 12u8, {});
dma_trait_impl!(crate::spi::TxDma, SPI6, BDMA_CH7, 12u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA1_CH0, 11u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA1_CH1, 11u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA1_CH2, 11u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA1_CH3, 11u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA1_CH4, 11u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA1_CH5, 11u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA1_CH6, 11u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA1_CH7, 11u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA2_CH0, 11u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA2_CH1, 11u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA2_CH2, 11u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA2_CH3, 11u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA2_CH4, 11u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA2_CH5, 11u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA2_CH6, 11u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM1, DMA2_CH7, 11u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA1_CH0, 12u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA1_CH1, 12u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA1_CH2, 12u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA1_CH3, 12u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA1_CH4, 12u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA1_CH5, 12u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA1_CH6, 12u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA1_CH7, 12u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA2_CH0, 12u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA2_CH1, 12u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA2_CH2, 12u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA2_CH3, 12u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA2_CH4, 12u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA2_CH5, 12u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA2_CH6, 12u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM1, DMA2_CH7, 12u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA1_CH0, 13u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA1_CH1, 13u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA1_CH2, 13u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA1_CH3, 13u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA1_CH4, 13u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA1_CH5, 13u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA1_CH6, 13u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA1_CH7, 13u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA2_CH0, 13u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA2_CH1, 13u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA2_CH2, 13u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA2_CH3, 13u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA2_CH4, 13u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA2_CH5, 13u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA2_CH6, 13u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM1, DMA2_CH7, 13u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA1_CH0, 14u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA1_CH1, 14u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA1_CH2, 14u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA1_CH3, 14u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA1_CH4, 14u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA1_CH5, 14u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA1_CH6, 14u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA1_CH7, 14u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA2_CH0, 14u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA2_CH1, 14u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA2_CH2, 14u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA2_CH3, 14u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA2_CH4, 14u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA2_CH5, 14u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA2_CH6, 14u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM1, DMA2_CH7, 14u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA1_CH0, 15u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA1_CH1, 15u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA1_CH2, 15u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA1_CH3, 15u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA1_CH4, 15u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA1_CH5, 15u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA1_CH6, 15u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA1_CH7, 15u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA2_CH0, 15u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA2_CH1, 15u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA2_CH2, 15u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA2_CH3, 15u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA2_CH4, 15u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA2_CH5, 15u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA2_CH6, 15u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM1, DMA2_CH7, 15u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA1_CH0, 105u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA1_CH1, 105u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA1_CH2, 105u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA1_CH3, 105u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA1_CH4, 105u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA1_CH5, 105u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA1_CH6, 105u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA1_CH7, 105u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA2_CH0, 105u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA2_CH1, 105u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA2_CH2, 105u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA2_CH3, 105u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA2_CH4, 105u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA2_CH5, 105u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA2_CH6, 105u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM15, DMA2_CH7, 105u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA1_CH0, 106u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA1_CH1, 106u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA1_CH2, 106u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA1_CH3, 106u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA1_CH4, 106u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA1_CH5, 106u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA1_CH6, 106u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA1_CH7, 106u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA2_CH0, 106u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA2_CH1, 106u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA2_CH2, 106u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA2_CH3, 106u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA2_CH4, 106u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA2_CH5, 106u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA2_CH6, 106u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM15, DMA2_CH7, 106u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA1_CH0, 109u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA1_CH1, 109u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA1_CH2, 109u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA1_CH3, 109u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA1_CH4, 109u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA1_CH5, 109u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA1_CH6, 109u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA1_CH7, 109u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA2_CH0, 109u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA2_CH1, 109u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA2_CH2, 109u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA2_CH3, 109u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA2_CH4, 109u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA2_CH5, 109u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA2_CH6, 109u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM16, DMA2_CH7, 109u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA1_CH0, 110u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA1_CH1, 110u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA1_CH2, 110u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA1_CH3, 110u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA1_CH4, 110u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA1_CH5, 110u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA1_CH6, 110u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA1_CH7, 110u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA2_CH0, 110u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA2_CH1, 110u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA2_CH2, 110u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA2_CH3, 110u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA2_CH4, 110u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA2_CH5, 110u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA2_CH6, 110u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM16, DMA2_CH7, 110u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA1_CH0, 111u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA1_CH1, 111u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA1_CH2, 111u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA1_CH3, 111u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA1_CH4, 111u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA1_CH5, 111u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA1_CH6, 111u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA1_CH7, 111u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA2_CH0, 111u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA2_CH1, 111u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA2_CH2, 111u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA2_CH3, 111u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA2_CH4, 111u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA2_CH5, 111u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA2_CH6, 111u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM17, DMA2_CH7, 111u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA1_CH0, 112u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA1_CH1, 112u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA1_CH2, 112u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA1_CH3, 112u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA1_CH4, 112u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA1_CH5, 112u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA1_CH6, 112u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA1_CH7, 112u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA2_CH0, 112u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA2_CH1, 112u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA2_CH2, 112u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA2_CH3, 112u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA2_CH4, 112u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA2_CH5, 112u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA2_CH6, 112u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM17, DMA2_CH7, 112u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA1_CH0, 18u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA1_CH1, 18u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA1_CH2, 18u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA1_CH3, 18u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA1_CH4, 18u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA1_CH5, 18u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA1_CH6, 18u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA1_CH7, 18u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA2_CH0, 18u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA2_CH1, 18u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA2_CH2, 18u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA2_CH3, 18u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA2_CH4, 18u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA2_CH5, 18u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA2_CH6, 18u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM2, DMA2_CH7, 18u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA1_CH0, 19u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA1_CH1, 19u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA1_CH2, 19u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA1_CH3, 19u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA1_CH4, 19u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA1_CH5, 19u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA1_CH6, 19u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA1_CH7, 19u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA2_CH0, 19u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA2_CH1, 19u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA2_CH2, 19u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA2_CH3, 19u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA2_CH4, 19u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA2_CH5, 19u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA2_CH6, 19u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM2, DMA2_CH7, 19u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA1_CH0, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA1_CH1, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA1_CH2, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA1_CH3, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA1_CH4, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA1_CH5, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA1_CH6, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA1_CH7, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA2_CH0, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA2_CH1, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA2_CH2, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA2_CH3, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA2_CH4, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA2_CH5, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA2_CH6, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM2, DMA2_CH7, 20u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA1_CH0, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA1_CH1, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA1_CH2, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA1_CH3, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA1_CH4, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA1_CH5, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA1_CH6, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA1_CH7, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA2_CH0, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA2_CH1, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA2_CH2, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA2_CH3, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA2_CH4, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA2_CH5, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA2_CH6, 21u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM2, DMA2_CH7, 21u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH0, 22u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH1, 22u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH2, 22u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH3, 22u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH4, 22u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH5, 22u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH6, 22u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA1_CH7, 22u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA2_CH0, 22u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA2_CH1, 22u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA2_CH2, 22u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA2_CH3, 22u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA2_CH4, 22u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA2_CH5, 22u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA2_CH6, 22u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM2, DMA2_CH7, 22u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA1_CH0, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA1_CH1, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA1_CH2, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA1_CH3, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA1_CH4, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA1_CH5, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA1_CH6, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA1_CH7, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA2_CH0, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA2_CH1, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA2_CH2, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA2_CH3, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA2_CH4, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA2_CH5, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA2_CH6, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM3, DMA2_CH7, 23u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA1_CH0, 24u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA1_CH1, 24u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA1_CH2, 24u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA1_CH3, 24u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA1_CH4, 24u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA1_CH5, 24u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA1_CH6, 24u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA1_CH7, 24u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA2_CH0, 24u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA2_CH1, 24u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA2_CH2, 24u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA2_CH3, 24u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA2_CH4, 24u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA2_CH5, 24u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA2_CH6, 24u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM3, DMA2_CH7, 24u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA1_CH0, 25u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA1_CH1, 25u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA1_CH2, 25u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA1_CH3, 25u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA1_CH4, 25u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA1_CH5, 25u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA1_CH6, 25u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA1_CH7, 25u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA2_CH0, 25u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA2_CH1, 25u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA2_CH2, 25u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA2_CH3, 25u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA2_CH4, 25u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA2_CH5, 25u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA2_CH6, 25u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM3, DMA2_CH7, 25u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA1_CH0, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA1_CH1, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA1_CH2, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA1_CH3, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA1_CH4, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA1_CH5, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA1_CH6, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA1_CH7, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA2_CH0, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA2_CH1, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA2_CH2, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA2_CH3, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA2_CH4, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA2_CH5, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA2_CH6, 26u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM3, DMA2_CH7, 26u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA1_CH0, 27u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA1_CH1, 27u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA1_CH2, 27u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA1_CH3, 27u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA1_CH4, 27u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA1_CH5, 27u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA1_CH6, 27u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA1_CH7, 27u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA2_CH0, 27u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA2_CH1, 27u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA2_CH2, 27u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA2_CH3, 27u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA2_CH4, 27u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA2_CH5, 27u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA2_CH6, 27u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM3, DMA2_CH7, 27u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA1_CH0, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA1_CH1, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA1_CH2, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA1_CH3, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA1_CH4, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA1_CH5, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA1_CH6, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA1_CH7, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA2_CH0, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA2_CH1, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA2_CH2, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA2_CH3, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA2_CH4, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA2_CH5, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA2_CH6, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM4, DMA2_CH7, 29u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA1_CH0, 30u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA1_CH1, 30u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA1_CH2, 30u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA1_CH3, 30u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA1_CH4, 30u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA1_CH5, 30u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA1_CH6, 30u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA1_CH7, 30u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA2_CH0, 30u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA2_CH1, 30u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA2_CH2, 30u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA2_CH3, 30u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA2_CH4, 30u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA2_CH5, 30u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA2_CH6, 30u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM4, DMA2_CH7, 30u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA1_CH0, 31u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA1_CH1, 31u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA1_CH2, 31u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA1_CH3, 31u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA1_CH4, 31u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA1_CH5, 31u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA1_CH6, 31u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA1_CH7, 31u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA2_CH0, 31u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA2_CH1, 31u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA2_CH2, 31u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA2_CH3, 31u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA2_CH4, 31u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA2_CH5, 31u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA2_CH6, 31u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM4, DMA2_CH7, 31u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA1_CH0, 32u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA1_CH1, 32u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA1_CH2, 32u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA1_CH3, 32u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA1_CH4, 32u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA1_CH5, 32u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA1_CH6, 32u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA1_CH7, 32u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA2_CH0, 32u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA2_CH1, 32u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA2_CH2, 32u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA2_CH3, 32u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA2_CH4, 32u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA2_CH5, 32u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA2_CH6, 32u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM4, DMA2_CH7, 32u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM5, DMA1_CH0, 55u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM5, DMA1_CH1, 55u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM5, DMA1_CH2, 55u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM5, DMA1_CH3, 55u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM5, DMA1_CH4, 55u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM5, DMA1_CH5, 55u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM5, DMA1_CH6, 55u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM5, DMA1_CH7, 55u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM5, DMA2_CH0, 55u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM5, DMA2_CH1, 55u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM5, DMA2_CH2, 55u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM5, DMA2_CH3, 55u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM5, DMA2_CH4, 55u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM5, DMA2_CH5, 55u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM5, DMA2_CH6, 55u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM5, DMA2_CH7, 55u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM5, DMA1_CH0, 56u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM5, DMA1_CH1, 56u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM5, DMA1_CH2, 56u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM5, DMA1_CH3, 56u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM5, DMA1_CH4, 56u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM5, DMA1_CH5, 56u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM5, DMA1_CH6, 56u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM5, DMA1_CH7, 56u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM5, DMA2_CH0, 56u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM5, DMA2_CH1, 56u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM5, DMA2_CH2, 56u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM5, DMA2_CH3, 56u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM5, DMA2_CH4, 56u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM5, DMA2_CH5, 56u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM5, DMA2_CH6, 56u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM5, DMA2_CH7, 56u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM5, DMA1_CH0, 57u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM5, DMA1_CH1, 57u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM5, DMA1_CH2, 57u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM5, DMA1_CH3, 57u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM5, DMA1_CH4, 57u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM5, DMA1_CH5, 57u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM5, DMA1_CH6, 57u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM5, DMA1_CH7, 57u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM5, DMA2_CH0, 57u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM5, DMA2_CH1, 57u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM5, DMA2_CH2, 57u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM5, DMA2_CH3, 57u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM5, DMA2_CH4, 57u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM5, DMA2_CH5, 57u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM5, DMA2_CH6, 57u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM5, DMA2_CH7, 57u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM5, DMA1_CH0, 58u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM5, DMA1_CH1, 58u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM5, DMA1_CH2, 58u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM5, DMA1_CH3, 58u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM5, DMA1_CH4, 58u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM5, DMA1_CH5, 58u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM5, DMA1_CH6, 58u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM5, DMA1_CH7, 58u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM5, DMA2_CH0, 58u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM5, DMA2_CH1, 58u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM5, DMA2_CH2, 58u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM5, DMA2_CH3, 58u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM5, DMA2_CH4, 58u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM5, DMA2_CH5, 58u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM5, DMA2_CH6, 58u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM5, DMA2_CH7, 58u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM5, DMA1_CH0, 59u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM5, DMA1_CH1, 59u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM5, DMA1_CH2, 59u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM5, DMA1_CH3, 59u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM5, DMA1_CH4, 59u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM5, DMA1_CH5, 59u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM5, DMA1_CH6, 59u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM5, DMA1_CH7, 59u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM5, DMA2_CH0, 59u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM5, DMA2_CH1, 59u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM5, DMA2_CH2, 59u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM5, DMA2_CH3, 59u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM5, DMA2_CH4, 59u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM5, DMA2_CH5, 59u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM5, DMA2_CH6, 59u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM5, DMA2_CH7, 59u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA1_CH0, 69u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA1_CH1, 69u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA1_CH2, 69u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA1_CH3, 69u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA1_CH4, 69u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA1_CH5, 69u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA1_CH6, 69u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA1_CH7, 69u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA2_CH0, 69u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA2_CH1, 69u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA2_CH2, 69u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA2_CH3, 69u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA2_CH4, 69u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA2_CH5, 69u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA2_CH6, 69u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM6, DMA2_CH7, 69u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA1_CH0, 70u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA1_CH1, 70u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA1_CH2, 70u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA1_CH3, 70u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA1_CH4, 70u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA1_CH5, 70u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA1_CH6, 70u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA1_CH7, 70u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA2_CH0, 70u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA2_CH1, 70u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA2_CH2, 70u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA2_CH3, 70u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA2_CH4, 70u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA2_CH5, 70u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA2_CH6, 70u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM7, DMA2_CH7, 70u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM8, DMA1_CH0, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM8, DMA1_CH1, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM8, DMA1_CH2, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM8, DMA1_CH3, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM8, DMA1_CH4, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM8, DMA1_CH5, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM8, DMA1_CH6, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM8, DMA1_CH7, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM8, DMA2_CH0, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM8, DMA2_CH1, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM8, DMA2_CH2, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM8, DMA2_CH3, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM8, DMA2_CH4, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM8, DMA2_CH5, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM8, DMA2_CH6, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch1>, TIM8, DMA2_CH7, 47u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM8, DMA1_CH0, 48u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM8, DMA1_CH1, 48u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM8, DMA1_CH2, 48u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM8, DMA1_CH3, 48u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM8, DMA1_CH4, 48u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM8, DMA1_CH5, 48u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM8, DMA1_CH6, 48u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM8, DMA1_CH7, 48u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM8, DMA2_CH0, 48u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM8, DMA2_CH1, 48u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM8, DMA2_CH2, 48u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM8, DMA2_CH3, 48u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM8, DMA2_CH4, 48u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM8, DMA2_CH5, 48u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM8, DMA2_CH6, 48u8, {});
dma_trait_impl!(crate::timer::Dma<Ch2>, TIM8, DMA2_CH7, 48u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM8, DMA1_CH0, 49u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM8, DMA1_CH1, 49u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM8, DMA1_CH2, 49u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM8, DMA1_CH3, 49u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM8, DMA1_CH4, 49u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM8, DMA1_CH5, 49u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM8, DMA1_CH6, 49u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM8, DMA1_CH7, 49u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM8, DMA2_CH0, 49u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM8, DMA2_CH1, 49u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM8, DMA2_CH2, 49u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM8, DMA2_CH3, 49u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM8, DMA2_CH4, 49u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM8, DMA2_CH5, 49u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM8, DMA2_CH6, 49u8, {});
dma_trait_impl!(crate::timer::Dma<Ch3>, TIM8, DMA2_CH7, 49u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM8, DMA1_CH0, 50u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM8, DMA1_CH1, 50u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM8, DMA1_CH2, 50u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM8, DMA1_CH3, 50u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM8, DMA1_CH4, 50u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM8, DMA1_CH5, 50u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM8, DMA1_CH6, 50u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM8, DMA1_CH7, 50u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM8, DMA2_CH0, 50u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM8, DMA2_CH1, 50u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM8, DMA2_CH2, 50u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM8, DMA2_CH3, 50u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM8, DMA2_CH4, 50u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM8, DMA2_CH5, 50u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM8, DMA2_CH6, 50u8, {});
dma_trait_impl!(crate::timer::Dma<Ch4>, TIM8, DMA2_CH7, 50u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM8, DMA1_CH0, 51u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM8, DMA1_CH1, 51u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM8, DMA1_CH2, 51u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM8, DMA1_CH3, 51u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM8, DMA1_CH4, 51u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM8, DMA1_CH5, 51u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM8, DMA1_CH6, 51u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM8, DMA1_CH7, 51u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM8, DMA2_CH0, 51u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM8, DMA2_CH1, 51u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM8, DMA2_CH2, 51u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM8, DMA2_CH3, 51u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM8, DMA2_CH4, 51u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM8, DMA2_CH5, 51u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM8, DMA2_CH6, 51u8, {});
dma_trait_impl!(crate::timer::UpDma, TIM8, DMA2_CH7, 51u8, {});
dma_trait_impl!(crate::usart::RxDma, UART4, DMA1_CH0, 63u8, {});
dma_trait_impl!(crate::usart::RxDma, UART4, DMA1_CH1, 63u8, {});
dma_trait_impl!(crate::usart::RxDma, UART4, DMA1_CH2, 63u8, {});
dma_trait_impl!(crate::usart::RxDma, UART4, DMA1_CH3, 63u8, {});
dma_trait_impl!(crate::usart::RxDma, UART4, DMA1_CH4, 63u8, {});
dma_trait_impl!(crate::usart::RxDma, UART4, DMA1_CH5, 63u8, {});
dma_trait_impl!(crate::usart::RxDma, UART4, DMA1_CH6, 63u8, {});
dma_trait_impl!(crate::usart::RxDma, UART4, DMA1_CH7, 63u8, {});
dma_trait_impl!(crate::usart::RxDma, UART4, DMA2_CH0, 63u8, {});
dma_trait_impl!(crate::usart::RxDma, UART4, DMA2_CH1, 63u8, {});
dma_trait_impl!(crate::usart::RxDma, UART4, DMA2_CH2, 63u8, {});
dma_trait_impl!(crate::usart::RxDma, UART4, DMA2_CH3, 63u8, {});
dma_trait_impl!(crate::usart::RxDma, UART4, DMA2_CH4, 63u8, {});
dma_trait_impl!(crate::usart::RxDma, UART4, DMA2_CH5, 63u8, {});
dma_trait_impl!(crate::usart::RxDma, UART4, DMA2_CH6, 63u8, {});
dma_trait_impl!(crate::usart::RxDma, UART4, DMA2_CH7, 63u8, {});
dma_trait_impl!(crate::usart::TxDma, UART4, DMA1_CH0, 64u8, {});
dma_trait_impl!(crate::usart::TxDma, UART4, DMA1_CH1, 64u8, {});
dma_trait_impl!(crate::usart::TxDma, UART4, DMA1_CH2, 64u8, {});
dma_trait_impl!(crate::usart::TxDma, UART4, DMA1_CH3, 64u8, {});
dma_trait_impl!(crate::usart::TxDma, UART4, DMA1_CH4, 64u8, {});
dma_trait_impl!(crate::usart::TxDma, UART4, DMA1_CH5, 64u8, {});
dma_trait_impl!(crate::usart::TxDma, UART4, DMA1_CH6, 64u8, {});
dma_trait_impl!(crate::usart::TxDma, UART4, DMA1_CH7, 64u8, {});
dma_trait_impl!(crate::usart::TxDma, UART4, DMA2_CH0, 64u8, {});
dma_trait_impl!(crate::usart::TxDma, UART4, DMA2_CH1, 64u8, {});
dma_trait_impl!(crate::usart::TxDma, UART4, DMA2_CH2, 64u8, {});
dma_trait_impl!(crate::usart::TxDma, UART4, DMA2_CH3, 64u8, {});
dma_trait_impl!(crate::usart::TxDma, UART4, DMA2_CH4, 64u8, {});
dma_trait_impl!(crate::usart::TxDma, UART4, DMA2_CH5, 64u8, {});
dma_trait_impl!(crate::usart::TxDma, UART4, DMA2_CH6, 64u8, {});
dma_trait_impl!(crate::usart::TxDma, UART4, DMA2_CH7, 64u8, {});
dma_trait_impl!(crate::usart::RxDma, UART5, DMA1_CH0, 65u8, {});
dma_trait_impl!(crate::usart::RxDma, UART5, DMA1_CH1, 65u8, {});
dma_trait_impl!(crate::usart::RxDma, UART5, DMA1_CH2, 65u8, {});
dma_trait_impl!(crate::usart::RxDma, UART5, DMA1_CH3, 65u8, {});
dma_trait_impl!(crate::usart::RxDma, UART5, DMA1_CH4, 65u8, {});
dma_trait_impl!(crate::usart::RxDma, UART5, DMA1_CH5, 65u8, {});
dma_trait_impl!(crate::usart::RxDma, UART5, DMA1_CH6, 65u8, {});
dma_trait_impl!(crate::usart::RxDma, UART5, DMA1_CH7, 65u8, {});
dma_trait_impl!(crate::usart::RxDma, UART5, DMA2_CH0, 65u8, {});
dma_trait_impl!(crate::usart::RxDma, UART5, DMA2_CH1, 65u8, {});
dma_trait_impl!(crate::usart::RxDma, UART5, DMA2_CH2, 65u8, {});
dma_trait_impl!(crate::usart::RxDma, UART5, DMA2_CH3, 65u8, {});
dma_trait_impl!(crate::usart::RxDma, UART5, DMA2_CH4, 65u8, {});
dma_trait_impl!(crate::usart::RxDma, UART5, DMA2_CH5, 65u8, {});
dma_trait_impl!(crate::usart::RxDma, UART5, DMA2_CH6, 65u8, {});
dma_trait_impl!(crate::usart::RxDma, UART5, DMA2_CH7, 65u8, {});
dma_trait_impl!(crate::usart::TxDma, UART5, DMA1_CH0, 66u8, {});
dma_trait_impl!(crate::usart::TxDma, UART5, DMA1_CH1, 66u8, {});
dma_trait_impl!(crate::usart::TxDma, UART5, DMA1_CH2, 66u8, {});
dma_trait_impl!(crate::usart::TxDma, UART5, DMA1_CH3, 66u8, {});
dma_trait_impl!(crate::usart::TxDma, UART5, DMA1_CH4, 66u8, {});
dma_trait_impl!(crate::usart::TxDma, UART5, DMA1_CH5, 66u8, {});
dma_trait_impl!(crate::usart::TxDma, UART5, DMA1_CH6, 66u8, {});
dma_trait_impl!(crate::usart::TxDma, UART5, DMA1_CH7, 66u8, {});
dma_trait_impl!(crate::usart::TxDma, UART5, DMA2_CH0, 66u8, {});
dma_trait_impl!(crate::usart::TxDma, UART5, DMA2_CH1, 66u8, {});
dma_trait_impl!(crate::usart::TxDma, UART5, DMA2_CH2, 66u8, {});
dma_trait_impl!(crate::usart::TxDma, UART5, DMA2_CH3, 66u8, {});
dma_trait_impl!(crate::usart::TxDma, UART5, DMA2_CH4, 66u8, {});
dma_trait_impl!(crate::usart::TxDma, UART5, DMA2_CH5, 66u8, {});
dma_trait_impl!(crate::usart::TxDma, UART5, DMA2_CH6, 66u8, {});
dma_trait_impl!(crate::usart::TxDma, UART5, DMA2_CH7, 66u8, {});
dma_trait_impl!(crate::usart::RxDma, UART7, DMA1_CH0, 79u8, {});
dma_trait_impl!(crate::usart::RxDma, UART7, DMA1_CH1, 79u8, {});
dma_trait_impl!(crate::usart::RxDma, UART7, DMA1_CH2, 79u8, {});
dma_trait_impl!(crate::usart::RxDma, UART7, DMA1_CH3, 79u8, {});
dma_trait_impl!(crate::usart::RxDma, UART7, DMA1_CH4, 79u8, {});
dma_trait_impl!(crate::usart::RxDma, UART7, DMA1_CH5, 79u8, {});
dma_trait_impl!(crate::usart::RxDma, UART7, DMA1_CH6, 79u8, {});
dma_trait_impl!(crate::usart::RxDma, UART7, DMA1_CH7, 79u8, {});
dma_trait_impl!(crate::usart::RxDma, UART7, DMA2_CH0, 79u8, {});
dma_trait_impl!(crate::usart::RxDma, UART7, DMA2_CH1, 79u8, {});
dma_trait_impl!(crate::usart::RxDma, UART7, DMA2_CH2, 79u8, {});
dma_trait_impl!(crate::usart::RxDma, UART7, DMA2_CH3, 79u8, {});
dma_trait_impl!(crate::usart::RxDma, UART7, DMA2_CH4, 79u8, {});
dma_trait_impl!(crate::usart::RxDma, UART7, DMA2_CH5, 79u8, {});
dma_trait_impl!(crate::usart::RxDma, UART7, DMA2_CH6, 79u8, {});
dma_trait_impl!(crate::usart::RxDma, UART7, DMA2_CH7, 79u8, {});
dma_trait_impl!(crate::usart::TxDma, UART7, DMA1_CH0, 80u8, {});
dma_trait_impl!(crate::usart::TxDma, UART7, DMA1_CH1, 80u8, {});
dma_trait_impl!(crate::usart::TxDma, UART7, DMA1_CH2, 80u8, {});
dma_trait_impl!(crate::usart::TxDma, UART7, DMA1_CH3, 80u8, {});
dma_trait_impl!(crate::usart::TxDma, UART7, DMA1_CH4, 80u8, {});
dma_trait_impl!(crate::usart::TxDma, UART7, DMA1_CH5, 80u8, {});
dma_trait_impl!(crate::usart::TxDma, UART7, DMA1_CH6, 80u8, {});
dma_trait_impl!(crate::usart::TxDma, UART7, DMA1_CH7, 80u8, {});
dma_trait_impl!(crate::usart::TxDma, UART7, DMA2_CH0, 80u8, {});
dma_trait_impl!(crate::usart::TxDma, UART7, DMA2_CH1, 80u8, {});
dma_trait_impl!(crate::usart::TxDma, UART7, DMA2_CH2, 80u8, {});
dma_trait_impl!(crate::usart::TxDma, UART7, DMA2_CH3, 80u8, {});
dma_trait_impl!(crate::usart::TxDma, UART7, DMA2_CH4, 80u8, {});
dma_trait_impl!(crate::usart::TxDma, UART7, DMA2_CH5, 80u8, {});
dma_trait_impl!(crate::usart::TxDma, UART7, DMA2_CH6, 80u8, {});
dma_trait_impl!(crate::usart::TxDma, UART7, DMA2_CH7, 80u8, {});
dma_trait_impl!(crate::usart::RxDma, UART8, DMA1_CH0, 81u8, {});
dma_trait_impl!(crate::usart::RxDma, UART8, DMA1_CH1, 81u8, {});
dma_trait_impl!(crate::usart::RxDma, UART8, DMA1_CH2, 81u8, {});
dma_trait_impl!(crate::usart::RxDma, UART8, DMA1_CH3, 81u8, {});
dma_trait_impl!(crate::usart::RxDma, UART8, DMA1_CH4, 81u8, {});
dma_trait_impl!(crate::usart::RxDma, UART8, DMA1_CH5, 81u8, {});
dma_trait_impl!(crate::usart::RxDma, UART8, DMA1_CH6, 81u8, {});
dma_trait_impl!(crate::usart::RxDma, UART8, DMA1_CH7, 81u8, {});
dma_trait_impl!(crate::usart::RxDma, UART8, DMA2_CH0, 81u8, {});
dma_trait_impl!(crate::usart::RxDma, UART8, DMA2_CH1, 81u8, {});
dma_trait_impl!(crate::usart::RxDma, UART8, DMA2_CH2, 81u8, {});
dma_trait_impl!(crate::usart::RxDma, UART8, DMA2_CH3, 81u8, {});
dma_trait_impl!(crate::usart::RxDma, UART8, DMA2_CH4, 81u8, {});
dma_trait_impl!(crate::usart::RxDma, UART8, DMA2_CH5, 81u8, {});
dma_trait_impl!(crate::usart::RxDma, UART8, DMA2_CH6, 81u8, {});
dma_trait_impl!(crate::usart::RxDma, UART8, DMA2_CH7, 81u8, {});
dma_trait_impl!(crate::usart::TxDma, UART8, DMA1_CH0, 82u8, {});
dma_trait_impl!(crate::usart::TxDma, UART8, DMA1_CH1, 82u8, {});
dma_trait_impl!(crate::usart::TxDma, UART8, DMA1_CH2, 82u8, {});
dma_trait_impl!(crate::usart::TxDma, UART8, DMA1_CH3, 82u8, {});
dma_trait_impl!(crate::usart::TxDma, UART8, DMA1_CH4, 82u8, {});
dma_trait_impl!(crate::usart::TxDma, UART8, DMA1_CH5, 82u8, {});
dma_trait_impl!(crate::usart::TxDma, UART8, DMA1_CH6, 82u8, {});
dma_trait_impl!(crate::usart::TxDma, UART8, DMA1_CH7, 82u8, {});
dma_trait_impl!(crate::usart::TxDma, UART8, DMA2_CH0, 82u8, {});
dma_trait_impl!(crate::usart::TxDma, UART8, DMA2_CH1, 82u8, {});
dma_trait_impl!(crate::usart::TxDma, UART8, DMA2_CH2, 82u8, {});
dma_trait_impl!(crate::usart::TxDma, UART8, DMA2_CH3, 82u8, {});
dma_trait_impl!(crate::usart::TxDma, UART8, DMA2_CH4, 82u8, {});
dma_trait_impl!(crate::usart::TxDma, UART8, DMA2_CH5, 82u8, {});
dma_trait_impl!(crate::usart::TxDma, UART8, DMA2_CH6, 82u8, {});
dma_trait_impl!(crate::usart::TxDma, UART8, DMA2_CH7, 82u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA1_CH0, 41u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA1_CH1, 41u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA1_CH2, 41u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA1_CH3, 41u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA1_CH4, 41u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA1_CH5, 41u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA1_CH6, 41u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA1_CH7, 41u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA2_CH0, 41u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA2_CH1, 41u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA2_CH2, 41u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA2_CH3, 41u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA2_CH4, 41u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA2_CH5, 41u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA2_CH6, 41u8, {});
dma_trait_impl!(crate::usart::RxDma, USART1, DMA2_CH7, 41u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA1_CH0, 42u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA1_CH1, 42u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA1_CH2, 42u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA1_CH3, 42u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA1_CH4, 42u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA1_CH5, 42u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA1_CH6, 42u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA1_CH7, 42u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA2_CH0, 42u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA2_CH1, 42u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA2_CH2, 42u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA2_CH3, 42u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA2_CH4, 42u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA2_CH5, 42u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA2_CH6, 42u8, {});
dma_trait_impl!(crate::usart::TxDma, USART1, DMA2_CH7, 42u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA1_CH0, 43u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA1_CH1, 43u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA1_CH2, 43u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA1_CH3, 43u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA1_CH4, 43u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA1_CH5, 43u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA1_CH6, 43u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA1_CH7, 43u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA2_CH0, 43u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA2_CH1, 43u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA2_CH2, 43u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA2_CH3, 43u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA2_CH4, 43u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA2_CH5, 43u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA2_CH6, 43u8, {});
dma_trait_impl!(crate::usart::RxDma, USART2, DMA2_CH7, 43u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA1_CH0, 44u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA1_CH1, 44u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA1_CH2, 44u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA1_CH3, 44u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA1_CH4, 44u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA1_CH5, 44u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA1_CH6, 44u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA1_CH7, 44u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA2_CH0, 44u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA2_CH1, 44u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA2_CH2, 44u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA2_CH3, 44u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA2_CH4, 44u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA2_CH5, 44u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA2_CH6, 44u8, {});
dma_trait_impl!(crate::usart::TxDma, USART2, DMA2_CH7, 44u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA1_CH0, 45u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA1_CH1, 45u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA1_CH2, 45u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA1_CH3, 45u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA1_CH4, 45u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA1_CH5, 45u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA1_CH6, 45u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA1_CH7, 45u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA2_CH0, 45u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA2_CH1, 45u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA2_CH2, 45u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA2_CH3, 45u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA2_CH4, 45u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA2_CH5, 45u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA2_CH6, 45u8, {});
dma_trait_impl!(crate::usart::RxDma, USART3, DMA2_CH7, 45u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA1_CH0, 46u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA1_CH1, 46u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA1_CH2, 46u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA1_CH3, 46u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA1_CH4, 46u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA1_CH5, 46u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA1_CH6, 46u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA1_CH7, 46u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA2_CH0, 46u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA2_CH1, 46u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA2_CH2, 46u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA2_CH3, 46u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA2_CH4, 46u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA2_CH5, 46u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA2_CH6, 46u8, {});
dma_trait_impl!(crate::usart::TxDma, USART3, DMA2_CH7, 46u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA1_CH0, 71u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA1_CH1, 71u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA1_CH2, 71u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA1_CH3, 71u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA1_CH4, 71u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA1_CH5, 71u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA1_CH6, 71u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA1_CH7, 71u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA2_CH0, 71u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA2_CH1, 71u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA2_CH2, 71u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA2_CH3, 71u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA2_CH4, 71u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA2_CH5, 71u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA2_CH6, 71u8, {});
dma_trait_impl!(crate::usart::RxDma, USART6, DMA2_CH7, 71u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA1_CH0, 72u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA1_CH1, 72u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA1_CH2, 72u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA1_CH3, 72u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA1_CH4, 72u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA1_CH5, 72u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA1_CH6, 72u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA1_CH7, 72u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA2_CH0, 72u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA2_CH1, 72u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA2_CH2, 72u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA2_CH3, 72u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA2_CH4, 72u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA2_CH5, 72u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA2_CH6, 72u8, {});
dma_trait_impl!(crate::usart::TxDma, USART6, DMA2_CH7, 72u8, {});
pub mod triggers {
    #[allow(non_camel_case_types)]
    pub struct EXTI9_TRG;
    #[allow(non_camel_case_types)]
    pub struct HRTIM_DAC_TRG1;
    #[allow(non_camel_case_types)]
    pub struct HRTIM_DAC_TRG2;
    #[allow(non_camel_case_types)]
    pub struct LPTIM1_TRGO;
    #[allow(non_camel_case_types)]
    pub struct LPTIM2_TRGO;
    #[allow(non_camel_case_types)]
    pub struct TIM15_TRGO;
    #[allow(non_camel_case_types)]
    pub struct TIM1_TRGO;
    #[allow(non_camel_case_types)]
    pub struct TIM2_TRGO;
    #[allow(non_camel_case_types)]
    pub struct TIM4_TRGO;
    #[allow(non_camel_case_types)]
    pub struct TIM5_TRGO;
    #[allow(non_camel_case_types)]
    pub struct TIM6_TRGO;
    #[allow(non_camel_case_types)]
    pub struct TIM7_TRGO;
    #[allow(non_camel_case_types)]
    pub struct TIM8_TRGO;
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Hpre {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Hpre::DIV1 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV2 => 2u32,
            crate::pac::rcc::vals::Hpre::DIV4 => 4u32,
            crate::pac::rcc::vals::Hpre::DIV8 => 8u32,
            crate::pac::rcc::vals::Hpre::DIV16 => 16u32,
            crate::pac::rcc::vals::Hpre::DIV64 => 64u32,
            crate::pac::rcc::vals::Hpre::DIV128 => 128u32,
            crate::pac::rcc::vals::Hpre::DIV256 => 256u32,
            crate::pac::rcc::vals::Hpre::DIV512 => 512u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Hpre::DIV1 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV2 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV4 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV8 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV16 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV64 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV128 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV256 => 1u32,
            crate::pac::rcc::vals::Hpre::DIV512 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Hsidiv {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Hsidiv::DIV1 => 1u32,
            crate::pac::rcc::vals::Hsidiv::DIV2 => 2u32,
            crate::pac::rcc::vals::Hsidiv::DIV4 => 4u32,
            crate::pac::rcc::vals::Hsidiv::DIV8 => 8u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Hsidiv::DIV1 => 1u32,
            crate::pac::rcc::vals::Hsidiv::DIV2 => 1u32,
            crate::pac::rcc::vals::Hsidiv::DIV4 => 1u32,
            crate::pac::rcc::vals::Hsidiv::DIV8 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Mcopre {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Mcopre::DIV1 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV2 => 2u32,
            crate::pac::rcc::vals::Mcopre::DIV3 => 3u32,
            crate::pac::rcc::vals::Mcopre::DIV4 => 4u32,
            crate::pac::rcc::vals::Mcopre::DIV5 => 5u32,
            crate::pac::rcc::vals::Mcopre::DIV6 => 6u32,
            crate::pac::rcc::vals::Mcopre::DIV7 => 7u32,
            crate::pac::rcc::vals::Mcopre::DIV8 => 8u32,
            crate::pac::rcc::vals::Mcopre::DIV9 => 9u32,
            crate::pac::rcc::vals::Mcopre::DIV10 => 10u32,
            crate::pac::rcc::vals::Mcopre::DIV11 => 11u32,
            crate::pac::rcc::vals::Mcopre::DIV12 => 12u32,
            crate::pac::rcc::vals::Mcopre::DIV13 => 13u32,
            crate::pac::rcc::vals::Mcopre::DIV14 => 14u32,
            crate::pac::rcc::vals::Mcopre::DIV15 => 15u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Mcopre::DIV1 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV2 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV3 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV4 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV5 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV6 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV7 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV8 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV9 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV10 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV11 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV12 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV13 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV14 => 1u32,
            crate::pac::rcc::vals::Mcopre::DIV15 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Plldiv {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Plldiv::DIV1 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV2 => 2u32,
            crate::pac::rcc::vals::Plldiv::DIV3 => 3u32,
            crate::pac::rcc::vals::Plldiv::DIV4 => 4u32,
            crate::pac::rcc::vals::Plldiv::DIV5 => 5u32,
            crate::pac::rcc::vals::Plldiv::DIV6 => 6u32,
            crate::pac::rcc::vals::Plldiv::DIV7 => 7u32,
            crate::pac::rcc::vals::Plldiv::DIV8 => 8u32,
            crate::pac::rcc::vals::Plldiv::DIV9 => 9u32,
            crate::pac::rcc::vals::Plldiv::DIV10 => 10u32,
            crate::pac::rcc::vals::Plldiv::DIV11 => 11u32,
            crate::pac::rcc::vals::Plldiv::DIV12 => 12u32,
            crate::pac::rcc::vals::Plldiv::DIV13 => 13u32,
            crate::pac::rcc::vals::Plldiv::DIV14 => 14u32,
            crate::pac::rcc::vals::Plldiv::DIV15 => 15u32,
            crate::pac::rcc::vals::Plldiv::DIV16 => 16u32,
            crate::pac::rcc::vals::Plldiv::DIV17 => 17u32,
            crate::pac::rcc::vals::Plldiv::DIV18 => 18u32,
            crate::pac::rcc::vals::Plldiv::DIV19 => 19u32,
            crate::pac::rcc::vals::Plldiv::DIV20 => 20u32,
            crate::pac::rcc::vals::Plldiv::DIV21 => 21u32,
            crate::pac::rcc::vals::Plldiv::DIV22 => 22u32,
            crate::pac::rcc::vals::Plldiv::DIV23 => 23u32,
            crate::pac::rcc::vals::Plldiv::DIV24 => 24u32,
            crate::pac::rcc::vals::Plldiv::DIV25 => 25u32,
            crate::pac::rcc::vals::Plldiv::DIV26 => 26u32,
            crate::pac::rcc::vals::Plldiv::DIV27 => 27u32,
            crate::pac::rcc::vals::Plldiv::DIV28 => 28u32,
            crate::pac::rcc::vals::Plldiv::DIV29 => 29u32,
            crate::pac::rcc::vals::Plldiv::DIV30 => 30u32,
            crate::pac::rcc::vals::Plldiv::DIV31 => 31u32,
            crate::pac::rcc::vals::Plldiv::DIV32 => 32u32,
            crate::pac::rcc::vals::Plldiv::DIV33 => 33u32,
            crate::pac::rcc::vals::Plldiv::DIV34 => 34u32,
            crate::pac::rcc::vals::Plldiv::DIV35 => 35u32,
            crate::pac::rcc::vals::Plldiv::DIV36 => 36u32,
            crate::pac::rcc::vals::Plldiv::DIV37 => 37u32,
            crate::pac::rcc::vals::Plldiv::DIV38 => 38u32,
            crate::pac::rcc::vals::Plldiv::DIV39 => 39u32,
            crate::pac::rcc::vals::Plldiv::DIV40 => 40u32,
            crate::pac::rcc::vals::Plldiv::DIV41 => 41u32,
            crate::pac::rcc::vals::Plldiv::DIV42 => 42u32,
            crate::pac::rcc::vals::Plldiv::DIV43 => 43u32,
            crate::pac::rcc::vals::Plldiv::DIV44 => 44u32,
            crate::pac::rcc::vals::Plldiv::DIV45 => 45u32,
            crate::pac::rcc::vals::Plldiv::DIV46 => 46u32,
            crate::pac::rcc::vals::Plldiv::DIV47 => 47u32,
            crate::pac::rcc::vals::Plldiv::DIV48 => 48u32,
            crate::pac::rcc::vals::Plldiv::DIV49 => 49u32,
            crate::pac::rcc::vals::Plldiv::DIV50 => 50u32,
            crate::pac::rcc::vals::Plldiv::DIV51 => 51u32,
            crate::pac::rcc::vals::Plldiv::DIV52 => 52u32,
            crate::pac::rcc::vals::Plldiv::DIV53 => 53u32,
            crate::pac::rcc::vals::Plldiv::DIV54 => 54u32,
            crate::pac::rcc::vals::Plldiv::DIV55 => 55u32,
            crate::pac::rcc::vals::Plldiv::DIV56 => 56u32,
            crate::pac::rcc::vals::Plldiv::DIV57 => 57u32,
            crate::pac::rcc::vals::Plldiv::DIV58 => 58u32,
            crate::pac::rcc::vals::Plldiv::DIV59 => 59u32,
            crate::pac::rcc::vals::Plldiv::DIV60 => 60u32,
            crate::pac::rcc::vals::Plldiv::DIV61 => 61u32,
            crate::pac::rcc::vals::Plldiv::DIV62 => 62u32,
            crate::pac::rcc::vals::Plldiv::DIV63 => 63u32,
            crate::pac::rcc::vals::Plldiv::DIV64 => 64u32,
            crate::pac::rcc::vals::Plldiv::DIV65 => 65u32,
            crate::pac::rcc::vals::Plldiv::DIV66 => 66u32,
            crate::pac::rcc::vals::Plldiv::DIV67 => 67u32,
            crate::pac::rcc::vals::Plldiv::DIV68 => 68u32,
            crate::pac::rcc::vals::Plldiv::DIV69 => 69u32,
            crate::pac::rcc::vals::Plldiv::DIV70 => 70u32,
            crate::pac::rcc::vals::Plldiv::DIV71 => 71u32,
            crate::pac::rcc::vals::Plldiv::DIV72 => 72u32,
            crate::pac::rcc::vals::Plldiv::DIV73 => 73u32,
            crate::pac::rcc::vals::Plldiv::DIV74 => 74u32,
            crate::pac::rcc::vals::Plldiv::DIV75 => 75u32,
            crate::pac::rcc::vals::Plldiv::DIV76 => 76u32,
            crate::pac::rcc::vals::Plldiv::DIV77 => 77u32,
            crate::pac::rcc::vals::Plldiv::DIV78 => 78u32,
            crate::pac::rcc::vals::Plldiv::DIV79 => 79u32,
            crate::pac::rcc::vals::Plldiv::DIV80 => 80u32,
            crate::pac::rcc::vals::Plldiv::DIV81 => 81u32,
            crate::pac::rcc::vals::Plldiv::DIV82 => 82u32,
            crate::pac::rcc::vals::Plldiv::DIV83 => 83u32,
            crate::pac::rcc::vals::Plldiv::DIV84 => 84u32,
            crate::pac::rcc::vals::Plldiv::DIV85 => 85u32,
            crate::pac::rcc::vals::Plldiv::DIV86 => 86u32,
            crate::pac::rcc::vals::Plldiv::DIV87 => 87u32,
            crate::pac::rcc::vals::Plldiv::DIV88 => 88u32,
            crate::pac::rcc::vals::Plldiv::DIV89 => 89u32,
            crate::pac::rcc::vals::Plldiv::DIV90 => 90u32,
            crate::pac::rcc::vals::Plldiv::DIV91 => 91u32,
            crate::pac::rcc::vals::Plldiv::DIV92 => 92u32,
            crate::pac::rcc::vals::Plldiv::DIV93 => 93u32,
            crate::pac::rcc::vals::Plldiv::DIV94 => 94u32,
            crate::pac::rcc::vals::Plldiv::DIV95 => 95u32,
            crate::pac::rcc::vals::Plldiv::DIV96 => 96u32,
            crate::pac::rcc::vals::Plldiv::DIV97 => 97u32,
            crate::pac::rcc::vals::Plldiv::DIV98 => 98u32,
            crate::pac::rcc::vals::Plldiv::DIV99 => 99u32,
            crate::pac::rcc::vals::Plldiv::DIV100 => 100u32,
            crate::pac::rcc::vals::Plldiv::DIV101 => 101u32,
            crate::pac::rcc::vals::Plldiv::DIV102 => 102u32,
            crate::pac::rcc::vals::Plldiv::DIV103 => 103u32,
            crate::pac::rcc::vals::Plldiv::DIV104 => 104u32,
            crate::pac::rcc::vals::Plldiv::DIV105 => 105u32,
            crate::pac::rcc::vals::Plldiv::DIV106 => 106u32,
            crate::pac::rcc::vals::Plldiv::DIV107 => 107u32,
            crate::pac::rcc::vals::Plldiv::DIV108 => 108u32,
            crate::pac::rcc::vals::Plldiv::DIV109 => 109u32,
            crate::pac::rcc::vals::Plldiv::DIV110 => 110u32,
            crate::pac::rcc::vals::Plldiv::DIV111 => 111u32,
            crate::pac::rcc::vals::Plldiv::DIV112 => 112u32,
            crate::pac::rcc::vals::Plldiv::DIV113 => 113u32,
            crate::pac::rcc::vals::Plldiv::DIV114 => 114u32,
            crate::pac::rcc::vals::Plldiv::DIV115 => 115u32,
            crate::pac::rcc::vals::Plldiv::DIV116 => 116u32,
            crate::pac::rcc::vals::Plldiv::DIV117 => 117u32,
            crate::pac::rcc::vals::Plldiv::DIV118 => 118u32,
            crate::pac::rcc::vals::Plldiv::DIV119 => 119u32,
            crate::pac::rcc::vals::Plldiv::DIV120 => 120u32,
            crate::pac::rcc::vals::Plldiv::DIV121 => 121u32,
            crate::pac::rcc::vals::Plldiv::DIV122 => 122u32,
            crate::pac::rcc::vals::Plldiv::DIV123 => 123u32,
            crate::pac::rcc::vals::Plldiv::DIV124 => 124u32,
            crate::pac::rcc::vals::Plldiv::DIV125 => 125u32,
            crate::pac::rcc::vals::Plldiv::DIV126 => 126u32,
            crate::pac::rcc::vals::Plldiv::DIV127 => 127u32,
            crate::pac::rcc::vals::Plldiv::DIV128 => 128u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Plldiv::DIV1 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV2 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV3 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV4 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV5 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV6 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV7 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV8 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV9 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV10 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV11 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV12 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV13 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV14 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV15 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV16 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV17 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV18 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV19 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV20 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV21 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV22 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV23 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV24 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV25 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV26 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV27 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV28 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV29 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV30 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV31 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV32 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV33 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV34 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV35 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV36 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV37 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV38 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV39 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV40 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV41 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV42 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV43 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV44 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV45 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV46 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV47 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV48 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV49 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV50 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV51 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV52 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV53 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV54 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV55 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV56 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV57 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV58 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV59 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV60 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV61 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV62 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV63 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV64 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV65 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV66 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV67 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV68 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV69 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV70 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV71 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV72 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV73 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV74 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV75 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV76 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV77 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV78 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV79 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV80 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV81 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV82 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV83 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV84 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV85 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV86 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV87 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV88 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV89 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV90 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV91 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV92 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV93 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV94 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV95 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV96 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV97 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV98 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV99 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV100 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV101 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV102 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV103 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV104 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV105 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV106 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV107 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV108 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV109 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV110 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV111 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV112 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV113 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV114 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV115 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV116 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV117 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV118 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV119 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV120 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV121 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV122 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV123 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV124 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV125 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV126 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV127 => 1u32,
            crate::pac::rcc::vals::Plldiv::DIV128 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Pllm {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Pllm::DIV1 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV2 => 2u32,
            crate::pac::rcc::vals::Pllm::DIV3 => 3u32,
            crate::pac::rcc::vals::Pllm::DIV4 => 4u32,
            crate::pac::rcc::vals::Pllm::DIV5 => 5u32,
            crate::pac::rcc::vals::Pllm::DIV6 => 6u32,
            crate::pac::rcc::vals::Pllm::DIV7 => 7u32,
            crate::pac::rcc::vals::Pllm::DIV8 => 8u32,
            crate::pac::rcc::vals::Pllm::DIV9 => 9u32,
            crate::pac::rcc::vals::Pllm::DIV10 => 10u32,
            crate::pac::rcc::vals::Pllm::DIV11 => 11u32,
            crate::pac::rcc::vals::Pllm::DIV12 => 12u32,
            crate::pac::rcc::vals::Pllm::DIV13 => 13u32,
            crate::pac::rcc::vals::Pllm::DIV14 => 14u32,
            crate::pac::rcc::vals::Pllm::DIV15 => 15u32,
            crate::pac::rcc::vals::Pllm::DIV16 => 16u32,
            crate::pac::rcc::vals::Pllm::DIV17 => 17u32,
            crate::pac::rcc::vals::Pllm::DIV18 => 18u32,
            crate::pac::rcc::vals::Pllm::DIV19 => 19u32,
            crate::pac::rcc::vals::Pllm::DIV20 => 20u32,
            crate::pac::rcc::vals::Pllm::DIV21 => 21u32,
            crate::pac::rcc::vals::Pllm::DIV22 => 22u32,
            crate::pac::rcc::vals::Pllm::DIV23 => 23u32,
            crate::pac::rcc::vals::Pllm::DIV24 => 24u32,
            crate::pac::rcc::vals::Pllm::DIV25 => 25u32,
            crate::pac::rcc::vals::Pllm::DIV26 => 26u32,
            crate::pac::rcc::vals::Pllm::DIV27 => 27u32,
            crate::pac::rcc::vals::Pllm::DIV28 => 28u32,
            crate::pac::rcc::vals::Pllm::DIV29 => 29u32,
            crate::pac::rcc::vals::Pllm::DIV30 => 30u32,
            crate::pac::rcc::vals::Pllm::DIV31 => 31u32,
            crate::pac::rcc::vals::Pllm::DIV32 => 32u32,
            crate::pac::rcc::vals::Pllm::DIV33 => 33u32,
            crate::pac::rcc::vals::Pllm::DIV34 => 34u32,
            crate::pac::rcc::vals::Pllm::DIV35 => 35u32,
            crate::pac::rcc::vals::Pllm::DIV36 => 36u32,
            crate::pac::rcc::vals::Pllm::DIV37 => 37u32,
            crate::pac::rcc::vals::Pllm::DIV38 => 38u32,
            crate::pac::rcc::vals::Pllm::DIV39 => 39u32,
            crate::pac::rcc::vals::Pllm::DIV40 => 40u32,
            crate::pac::rcc::vals::Pllm::DIV41 => 41u32,
            crate::pac::rcc::vals::Pllm::DIV42 => 42u32,
            crate::pac::rcc::vals::Pllm::DIV43 => 43u32,
            crate::pac::rcc::vals::Pllm::DIV44 => 44u32,
            crate::pac::rcc::vals::Pllm::DIV45 => 45u32,
            crate::pac::rcc::vals::Pllm::DIV46 => 46u32,
            crate::pac::rcc::vals::Pllm::DIV47 => 47u32,
            crate::pac::rcc::vals::Pllm::DIV48 => 48u32,
            crate::pac::rcc::vals::Pllm::DIV49 => 49u32,
            crate::pac::rcc::vals::Pllm::DIV50 => 50u32,
            crate::pac::rcc::vals::Pllm::DIV51 => 51u32,
            crate::pac::rcc::vals::Pllm::DIV52 => 52u32,
            crate::pac::rcc::vals::Pllm::DIV53 => 53u32,
            crate::pac::rcc::vals::Pllm::DIV54 => 54u32,
            crate::pac::rcc::vals::Pllm::DIV55 => 55u32,
            crate::pac::rcc::vals::Pllm::DIV56 => 56u32,
            crate::pac::rcc::vals::Pllm::DIV57 => 57u32,
            crate::pac::rcc::vals::Pllm::DIV58 => 58u32,
            crate::pac::rcc::vals::Pllm::DIV59 => 59u32,
            crate::pac::rcc::vals::Pllm::DIV60 => 60u32,
            crate::pac::rcc::vals::Pllm::DIV61 => 61u32,
            crate::pac::rcc::vals::Pllm::DIV62 => 62u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Pllm::DIV1 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV2 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV3 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV4 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV5 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV6 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV7 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV8 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV9 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV10 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV11 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV12 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV13 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV14 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV15 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV16 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV17 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV18 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV19 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV20 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV21 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV22 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV23 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV24 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV25 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV26 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV27 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV28 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV29 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV30 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV31 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV32 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV33 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV34 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV35 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV36 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV37 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV38 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV39 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV40 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV41 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV42 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV43 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV44 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV45 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV46 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV47 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV48 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV49 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV50 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV51 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV52 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV53 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV54 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV55 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV56 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV57 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV58 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV59 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV60 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV61 => 1u32,
            crate::pac::rcc::vals::Pllm::DIV62 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Plln {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Plln::MUL4 => 4u32,
            crate::pac::rcc::vals::Plln::MUL5 => 5u32,
            crate::pac::rcc::vals::Plln::MUL6 => 6u32,
            crate::pac::rcc::vals::Plln::MUL7 => 7u32,
            crate::pac::rcc::vals::Plln::MUL8 => 8u32,
            crate::pac::rcc::vals::Plln::MUL9 => 9u32,
            crate::pac::rcc::vals::Plln::MUL10 => 10u32,
            crate::pac::rcc::vals::Plln::MUL11 => 11u32,
            crate::pac::rcc::vals::Plln::MUL12 => 12u32,
            crate::pac::rcc::vals::Plln::MUL13 => 13u32,
            crate::pac::rcc::vals::Plln::MUL14 => 14u32,
            crate::pac::rcc::vals::Plln::MUL15 => 15u32,
            crate::pac::rcc::vals::Plln::MUL16 => 16u32,
            crate::pac::rcc::vals::Plln::MUL17 => 17u32,
            crate::pac::rcc::vals::Plln::MUL18 => 18u32,
            crate::pac::rcc::vals::Plln::MUL19 => 19u32,
            crate::pac::rcc::vals::Plln::MUL20 => 20u32,
            crate::pac::rcc::vals::Plln::MUL21 => 21u32,
            crate::pac::rcc::vals::Plln::MUL22 => 22u32,
            crate::pac::rcc::vals::Plln::MUL23 => 23u32,
            crate::pac::rcc::vals::Plln::MUL24 => 24u32,
            crate::pac::rcc::vals::Plln::MUL25 => 25u32,
            crate::pac::rcc::vals::Plln::MUL26 => 26u32,
            crate::pac::rcc::vals::Plln::MUL27 => 27u32,
            crate::pac::rcc::vals::Plln::MUL28 => 28u32,
            crate::pac::rcc::vals::Plln::MUL29 => 29u32,
            crate::pac::rcc::vals::Plln::MUL30 => 30u32,
            crate::pac::rcc::vals::Plln::MUL31 => 31u32,
            crate::pac::rcc::vals::Plln::MUL32 => 32u32,
            crate::pac::rcc::vals::Plln::MUL33 => 33u32,
            crate::pac::rcc::vals::Plln::MUL34 => 34u32,
            crate::pac::rcc::vals::Plln::MUL35 => 35u32,
            crate::pac::rcc::vals::Plln::MUL36 => 36u32,
            crate::pac::rcc::vals::Plln::MUL37 => 37u32,
            crate::pac::rcc::vals::Plln::MUL38 => 38u32,
            crate::pac::rcc::vals::Plln::MUL39 => 39u32,
            crate::pac::rcc::vals::Plln::MUL40 => 40u32,
            crate::pac::rcc::vals::Plln::MUL41 => 41u32,
            crate::pac::rcc::vals::Plln::MUL42 => 42u32,
            crate::pac::rcc::vals::Plln::MUL43 => 43u32,
            crate::pac::rcc::vals::Plln::MUL44 => 44u32,
            crate::pac::rcc::vals::Plln::MUL45 => 45u32,
            crate::pac::rcc::vals::Plln::MUL46 => 46u32,
            crate::pac::rcc::vals::Plln::MUL47 => 47u32,
            crate::pac::rcc::vals::Plln::MUL48 => 48u32,
            crate::pac::rcc::vals::Plln::MUL49 => 49u32,
            crate::pac::rcc::vals::Plln::MUL50 => 50u32,
            crate::pac::rcc::vals::Plln::MUL51 => 51u32,
            crate::pac::rcc::vals::Plln::MUL52 => 52u32,
            crate::pac::rcc::vals::Plln::MUL53 => 53u32,
            crate::pac::rcc::vals::Plln::MUL54 => 54u32,
            crate::pac::rcc::vals::Plln::MUL55 => 55u32,
            crate::pac::rcc::vals::Plln::MUL56 => 56u32,
            crate::pac::rcc::vals::Plln::MUL57 => 57u32,
            crate::pac::rcc::vals::Plln::MUL58 => 58u32,
            crate::pac::rcc::vals::Plln::MUL59 => 59u32,
            crate::pac::rcc::vals::Plln::MUL60 => 60u32,
            crate::pac::rcc::vals::Plln::MUL61 => 61u32,
            crate::pac::rcc::vals::Plln::MUL62 => 62u32,
            crate::pac::rcc::vals::Plln::MUL63 => 63u32,
            crate::pac::rcc::vals::Plln::MUL64 => 64u32,
            crate::pac::rcc::vals::Plln::MUL65 => 65u32,
            crate::pac::rcc::vals::Plln::MUL66 => 66u32,
            crate::pac::rcc::vals::Plln::MUL67 => 67u32,
            crate::pac::rcc::vals::Plln::MUL68 => 68u32,
            crate::pac::rcc::vals::Plln::MUL69 => 69u32,
            crate::pac::rcc::vals::Plln::MUL70 => 70u32,
            crate::pac::rcc::vals::Plln::MUL71 => 71u32,
            crate::pac::rcc::vals::Plln::MUL72 => 72u32,
            crate::pac::rcc::vals::Plln::MUL73 => 73u32,
            crate::pac::rcc::vals::Plln::MUL74 => 74u32,
            crate::pac::rcc::vals::Plln::MUL75 => 75u32,
            crate::pac::rcc::vals::Plln::MUL76 => 76u32,
            crate::pac::rcc::vals::Plln::MUL77 => 77u32,
            crate::pac::rcc::vals::Plln::MUL78 => 78u32,
            crate::pac::rcc::vals::Plln::MUL79 => 79u32,
            crate::pac::rcc::vals::Plln::MUL80 => 80u32,
            crate::pac::rcc::vals::Plln::MUL81 => 81u32,
            crate::pac::rcc::vals::Plln::MUL82 => 82u32,
            crate::pac::rcc::vals::Plln::MUL83 => 83u32,
            crate::pac::rcc::vals::Plln::MUL84 => 84u32,
            crate::pac::rcc::vals::Plln::MUL85 => 85u32,
            crate::pac::rcc::vals::Plln::MUL86 => 86u32,
            crate::pac::rcc::vals::Plln::MUL87 => 87u32,
            crate::pac::rcc::vals::Plln::MUL88 => 88u32,
            crate::pac::rcc::vals::Plln::MUL89 => 89u32,
            crate::pac::rcc::vals::Plln::MUL90 => 90u32,
            crate::pac::rcc::vals::Plln::MUL91 => 91u32,
            crate::pac::rcc::vals::Plln::MUL92 => 92u32,
            crate::pac::rcc::vals::Plln::MUL93 => 93u32,
            crate::pac::rcc::vals::Plln::MUL94 => 94u32,
            crate::pac::rcc::vals::Plln::MUL95 => 95u32,
            crate::pac::rcc::vals::Plln::MUL96 => 96u32,
            crate::pac::rcc::vals::Plln::MUL97 => 97u32,
            crate::pac::rcc::vals::Plln::MUL98 => 98u32,
            crate::pac::rcc::vals::Plln::MUL99 => 99u32,
            crate::pac::rcc::vals::Plln::MUL100 => 100u32,
            crate::pac::rcc::vals::Plln::MUL101 => 101u32,
            crate::pac::rcc::vals::Plln::MUL102 => 102u32,
            crate::pac::rcc::vals::Plln::MUL103 => 103u32,
            crate::pac::rcc::vals::Plln::MUL104 => 104u32,
            crate::pac::rcc::vals::Plln::MUL105 => 105u32,
            crate::pac::rcc::vals::Plln::MUL106 => 106u32,
            crate::pac::rcc::vals::Plln::MUL107 => 107u32,
            crate::pac::rcc::vals::Plln::MUL108 => 108u32,
            crate::pac::rcc::vals::Plln::MUL109 => 109u32,
            crate::pac::rcc::vals::Plln::MUL110 => 110u32,
            crate::pac::rcc::vals::Plln::MUL111 => 111u32,
            crate::pac::rcc::vals::Plln::MUL112 => 112u32,
            crate::pac::rcc::vals::Plln::MUL113 => 113u32,
            crate::pac::rcc::vals::Plln::MUL114 => 114u32,
            crate::pac::rcc::vals::Plln::MUL115 => 115u32,
            crate::pac::rcc::vals::Plln::MUL116 => 116u32,
            crate::pac::rcc::vals::Plln::MUL117 => 117u32,
            crate::pac::rcc::vals::Plln::MUL118 => 118u32,
            crate::pac::rcc::vals::Plln::MUL119 => 119u32,
            crate::pac::rcc::vals::Plln::MUL120 => 120u32,
            crate::pac::rcc::vals::Plln::MUL121 => 121u32,
            crate::pac::rcc::vals::Plln::MUL122 => 122u32,
            crate::pac::rcc::vals::Plln::MUL123 => 123u32,
            crate::pac::rcc::vals::Plln::MUL124 => 124u32,
            crate::pac::rcc::vals::Plln::MUL125 => 125u32,
            crate::pac::rcc::vals::Plln::MUL126 => 126u32,
            crate::pac::rcc::vals::Plln::MUL127 => 127u32,
            crate::pac::rcc::vals::Plln::MUL128 => 128u32,
            crate::pac::rcc::vals::Plln::MUL129 => 129u32,
            crate::pac::rcc::vals::Plln::MUL130 => 130u32,
            crate::pac::rcc::vals::Plln::MUL131 => 131u32,
            crate::pac::rcc::vals::Plln::MUL132 => 132u32,
            crate::pac::rcc::vals::Plln::MUL133 => 133u32,
            crate::pac::rcc::vals::Plln::MUL134 => 134u32,
            crate::pac::rcc::vals::Plln::MUL135 => 135u32,
            crate::pac::rcc::vals::Plln::MUL136 => 136u32,
            crate::pac::rcc::vals::Plln::MUL137 => 137u32,
            crate::pac::rcc::vals::Plln::MUL138 => 138u32,
            crate::pac::rcc::vals::Plln::MUL139 => 139u32,
            crate::pac::rcc::vals::Plln::MUL140 => 140u32,
            crate::pac::rcc::vals::Plln::MUL141 => 141u32,
            crate::pac::rcc::vals::Plln::MUL142 => 142u32,
            crate::pac::rcc::vals::Plln::MUL143 => 143u32,
            crate::pac::rcc::vals::Plln::MUL144 => 144u32,
            crate::pac::rcc::vals::Plln::MUL145 => 145u32,
            crate::pac::rcc::vals::Plln::MUL146 => 146u32,
            crate::pac::rcc::vals::Plln::MUL147 => 147u32,
            crate::pac::rcc::vals::Plln::MUL148 => 148u32,
            crate::pac::rcc::vals::Plln::MUL149 => 149u32,
            crate::pac::rcc::vals::Plln::MUL150 => 150u32,
            crate::pac::rcc::vals::Plln::MUL151 => 151u32,
            crate::pac::rcc::vals::Plln::MUL152 => 152u32,
            crate::pac::rcc::vals::Plln::MUL153 => 153u32,
            crate::pac::rcc::vals::Plln::MUL154 => 154u32,
            crate::pac::rcc::vals::Plln::MUL155 => 155u32,
            crate::pac::rcc::vals::Plln::MUL156 => 156u32,
            crate::pac::rcc::vals::Plln::MUL157 => 157u32,
            crate::pac::rcc::vals::Plln::MUL158 => 158u32,
            crate::pac::rcc::vals::Plln::MUL159 => 159u32,
            crate::pac::rcc::vals::Plln::MUL160 => 160u32,
            crate::pac::rcc::vals::Plln::MUL161 => 161u32,
            crate::pac::rcc::vals::Plln::MUL162 => 162u32,
            crate::pac::rcc::vals::Plln::MUL163 => 163u32,
            crate::pac::rcc::vals::Plln::MUL164 => 164u32,
            crate::pac::rcc::vals::Plln::MUL165 => 165u32,
            crate::pac::rcc::vals::Plln::MUL166 => 166u32,
            crate::pac::rcc::vals::Plln::MUL167 => 167u32,
            crate::pac::rcc::vals::Plln::MUL168 => 168u32,
            crate::pac::rcc::vals::Plln::MUL169 => 169u32,
            crate::pac::rcc::vals::Plln::MUL170 => 170u32,
            crate::pac::rcc::vals::Plln::MUL171 => 171u32,
            crate::pac::rcc::vals::Plln::MUL172 => 172u32,
            crate::pac::rcc::vals::Plln::MUL173 => 173u32,
            crate::pac::rcc::vals::Plln::MUL174 => 174u32,
            crate::pac::rcc::vals::Plln::MUL175 => 175u32,
            crate::pac::rcc::vals::Plln::MUL176 => 176u32,
            crate::pac::rcc::vals::Plln::MUL177 => 177u32,
            crate::pac::rcc::vals::Plln::MUL178 => 178u32,
            crate::pac::rcc::vals::Plln::MUL179 => 179u32,
            crate::pac::rcc::vals::Plln::MUL180 => 180u32,
            crate::pac::rcc::vals::Plln::MUL181 => 181u32,
            crate::pac::rcc::vals::Plln::MUL182 => 182u32,
            crate::pac::rcc::vals::Plln::MUL183 => 183u32,
            crate::pac::rcc::vals::Plln::MUL184 => 184u32,
            crate::pac::rcc::vals::Plln::MUL185 => 185u32,
            crate::pac::rcc::vals::Plln::MUL186 => 186u32,
            crate::pac::rcc::vals::Plln::MUL187 => 187u32,
            crate::pac::rcc::vals::Plln::MUL188 => 188u32,
            crate::pac::rcc::vals::Plln::MUL189 => 189u32,
            crate::pac::rcc::vals::Plln::MUL190 => 190u32,
            crate::pac::rcc::vals::Plln::MUL191 => 191u32,
            crate::pac::rcc::vals::Plln::MUL192 => 192u32,
            crate::pac::rcc::vals::Plln::MUL193 => 193u32,
            crate::pac::rcc::vals::Plln::MUL194 => 194u32,
            crate::pac::rcc::vals::Plln::MUL195 => 195u32,
            crate::pac::rcc::vals::Plln::MUL196 => 196u32,
            crate::pac::rcc::vals::Plln::MUL197 => 197u32,
            crate::pac::rcc::vals::Plln::MUL198 => 198u32,
            crate::pac::rcc::vals::Plln::MUL199 => 199u32,
            crate::pac::rcc::vals::Plln::MUL200 => 200u32,
            crate::pac::rcc::vals::Plln::MUL201 => 201u32,
            crate::pac::rcc::vals::Plln::MUL202 => 202u32,
            crate::pac::rcc::vals::Plln::MUL203 => 203u32,
            crate::pac::rcc::vals::Plln::MUL204 => 204u32,
            crate::pac::rcc::vals::Plln::MUL205 => 205u32,
            crate::pac::rcc::vals::Plln::MUL206 => 206u32,
            crate::pac::rcc::vals::Plln::MUL207 => 207u32,
            crate::pac::rcc::vals::Plln::MUL208 => 208u32,
            crate::pac::rcc::vals::Plln::MUL209 => 209u32,
            crate::pac::rcc::vals::Plln::MUL210 => 210u32,
            crate::pac::rcc::vals::Plln::MUL211 => 211u32,
            crate::pac::rcc::vals::Plln::MUL212 => 212u32,
            crate::pac::rcc::vals::Plln::MUL213 => 213u32,
            crate::pac::rcc::vals::Plln::MUL214 => 214u32,
            crate::pac::rcc::vals::Plln::MUL215 => 215u32,
            crate::pac::rcc::vals::Plln::MUL216 => 216u32,
            crate::pac::rcc::vals::Plln::MUL217 => 217u32,
            crate::pac::rcc::vals::Plln::MUL218 => 218u32,
            crate::pac::rcc::vals::Plln::MUL219 => 219u32,
            crate::pac::rcc::vals::Plln::MUL220 => 220u32,
            crate::pac::rcc::vals::Plln::MUL221 => 221u32,
            crate::pac::rcc::vals::Plln::MUL222 => 222u32,
            crate::pac::rcc::vals::Plln::MUL223 => 223u32,
            crate::pac::rcc::vals::Plln::MUL224 => 224u32,
            crate::pac::rcc::vals::Plln::MUL225 => 225u32,
            crate::pac::rcc::vals::Plln::MUL226 => 226u32,
            crate::pac::rcc::vals::Plln::MUL227 => 227u32,
            crate::pac::rcc::vals::Plln::MUL228 => 228u32,
            crate::pac::rcc::vals::Plln::MUL229 => 229u32,
            crate::pac::rcc::vals::Plln::MUL230 => 230u32,
            crate::pac::rcc::vals::Plln::MUL231 => 231u32,
            crate::pac::rcc::vals::Plln::MUL232 => 232u32,
            crate::pac::rcc::vals::Plln::MUL233 => 233u32,
            crate::pac::rcc::vals::Plln::MUL234 => 234u32,
            crate::pac::rcc::vals::Plln::MUL235 => 235u32,
            crate::pac::rcc::vals::Plln::MUL236 => 236u32,
            crate::pac::rcc::vals::Plln::MUL237 => 237u32,
            crate::pac::rcc::vals::Plln::MUL238 => 238u32,
            crate::pac::rcc::vals::Plln::MUL239 => 239u32,
            crate::pac::rcc::vals::Plln::MUL240 => 240u32,
            crate::pac::rcc::vals::Plln::MUL241 => 241u32,
            crate::pac::rcc::vals::Plln::MUL242 => 242u32,
            crate::pac::rcc::vals::Plln::MUL243 => 243u32,
            crate::pac::rcc::vals::Plln::MUL244 => 244u32,
            crate::pac::rcc::vals::Plln::MUL245 => 245u32,
            crate::pac::rcc::vals::Plln::MUL246 => 246u32,
            crate::pac::rcc::vals::Plln::MUL247 => 247u32,
            crate::pac::rcc::vals::Plln::MUL248 => 248u32,
            crate::pac::rcc::vals::Plln::MUL249 => 249u32,
            crate::pac::rcc::vals::Plln::MUL250 => 250u32,
            crate::pac::rcc::vals::Plln::MUL251 => 251u32,
            crate::pac::rcc::vals::Plln::MUL252 => 252u32,
            crate::pac::rcc::vals::Plln::MUL253 => 253u32,
            crate::pac::rcc::vals::Plln::MUL254 => 254u32,
            crate::pac::rcc::vals::Plln::MUL255 => 255u32,
            crate::pac::rcc::vals::Plln::MUL256 => 256u32,
            crate::pac::rcc::vals::Plln::MUL257 => 257u32,
            crate::pac::rcc::vals::Plln::MUL258 => 258u32,
            crate::pac::rcc::vals::Plln::MUL259 => 259u32,
            crate::pac::rcc::vals::Plln::MUL260 => 260u32,
            crate::pac::rcc::vals::Plln::MUL261 => 261u32,
            crate::pac::rcc::vals::Plln::MUL262 => 262u32,
            crate::pac::rcc::vals::Plln::MUL263 => 263u32,
            crate::pac::rcc::vals::Plln::MUL264 => 264u32,
            crate::pac::rcc::vals::Plln::MUL265 => 265u32,
            crate::pac::rcc::vals::Plln::MUL266 => 266u32,
            crate::pac::rcc::vals::Plln::MUL267 => 267u32,
            crate::pac::rcc::vals::Plln::MUL268 => 268u32,
            crate::pac::rcc::vals::Plln::MUL269 => 269u32,
            crate::pac::rcc::vals::Plln::MUL270 => 270u32,
            crate::pac::rcc::vals::Plln::MUL271 => 271u32,
            crate::pac::rcc::vals::Plln::MUL272 => 272u32,
            crate::pac::rcc::vals::Plln::MUL273 => 273u32,
            crate::pac::rcc::vals::Plln::MUL274 => 274u32,
            crate::pac::rcc::vals::Plln::MUL275 => 275u32,
            crate::pac::rcc::vals::Plln::MUL276 => 276u32,
            crate::pac::rcc::vals::Plln::MUL277 => 277u32,
            crate::pac::rcc::vals::Plln::MUL278 => 278u32,
            crate::pac::rcc::vals::Plln::MUL279 => 279u32,
            crate::pac::rcc::vals::Plln::MUL280 => 280u32,
            crate::pac::rcc::vals::Plln::MUL281 => 281u32,
            crate::pac::rcc::vals::Plln::MUL282 => 282u32,
            crate::pac::rcc::vals::Plln::MUL283 => 283u32,
            crate::pac::rcc::vals::Plln::MUL284 => 284u32,
            crate::pac::rcc::vals::Plln::MUL285 => 285u32,
            crate::pac::rcc::vals::Plln::MUL286 => 286u32,
            crate::pac::rcc::vals::Plln::MUL287 => 287u32,
            crate::pac::rcc::vals::Plln::MUL288 => 288u32,
            crate::pac::rcc::vals::Plln::MUL289 => 289u32,
            crate::pac::rcc::vals::Plln::MUL290 => 290u32,
            crate::pac::rcc::vals::Plln::MUL291 => 291u32,
            crate::pac::rcc::vals::Plln::MUL292 => 292u32,
            crate::pac::rcc::vals::Plln::MUL293 => 293u32,
            crate::pac::rcc::vals::Plln::MUL294 => 294u32,
            crate::pac::rcc::vals::Plln::MUL295 => 295u32,
            crate::pac::rcc::vals::Plln::MUL296 => 296u32,
            crate::pac::rcc::vals::Plln::MUL297 => 297u32,
            crate::pac::rcc::vals::Plln::MUL298 => 298u32,
            crate::pac::rcc::vals::Plln::MUL299 => 299u32,
            crate::pac::rcc::vals::Plln::MUL300 => 300u32,
            crate::pac::rcc::vals::Plln::MUL301 => 301u32,
            crate::pac::rcc::vals::Plln::MUL302 => 302u32,
            crate::pac::rcc::vals::Plln::MUL303 => 303u32,
            crate::pac::rcc::vals::Plln::MUL304 => 304u32,
            crate::pac::rcc::vals::Plln::MUL305 => 305u32,
            crate::pac::rcc::vals::Plln::MUL306 => 306u32,
            crate::pac::rcc::vals::Plln::MUL307 => 307u32,
            crate::pac::rcc::vals::Plln::MUL308 => 308u32,
            crate::pac::rcc::vals::Plln::MUL309 => 309u32,
            crate::pac::rcc::vals::Plln::MUL310 => 310u32,
            crate::pac::rcc::vals::Plln::MUL311 => 311u32,
            crate::pac::rcc::vals::Plln::MUL312 => 312u32,
            crate::pac::rcc::vals::Plln::MUL313 => 313u32,
            crate::pac::rcc::vals::Plln::MUL314 => 314u32,
            crate::pac::rcc::vals::Plln::MUL315 => 315u32,
            crate::pac::rcc::vals::Plln::MUL316 => 316u32,
            crate::pac::rcc::vals::Plln::MUL317 => 317u32,
            crate::pac::rcc::vals::Plln::MUL318 => 318u32,
            crate::pac::rcc::vals::Plln::MUL319 => 319u32,
            crate::pac::rcc::vals::Plln::MUL320 => 320u32,
            crate::pac::rcc::vals::Plln::MUL321 => 321u32,
            crate::pac::rcc::vals::Plln::MUL322 => 322u32,
            crate::pac::rcc::vals::Plln::MUL323 => 323u32,
            crate::pac::rcc::vals::Plln::MUL324 => 324u32,
            crate::pac::rcc::vals::Plln::MUL325 => 325u32,
            crate::pac::rcc::vals::Plln::MUL326 => 326u32,
            crate::pac::rcc::vals::Plln::MUL327 => 327u32,
            crate::pac::rcc::vals::Plln::MUL328 => 328u32,
            crate::pac::rcc::vals::Plln::MUL329 => 329u32,
            crate::pac::rcc::vals::Plln::MUL330 => 330u32,
            crate::pac::rcc::vals::Plln::MUL331 => 331u32,
            crate::pac::rcc::vals::Plln::MUL332 => 332u32,
            crate::pac::rcc::vals::Plln::MUL333 => 333u32,
            crate::pac::rcc::vals::Plln::MUL334 => 334u32,
            crate::pac::rcc::vals::Plln::MUL335 => 335u32,
            crate::pac::rcc::vals::Plln::MUL336 => 336u32,
            crate::pac::rcc::vals::Plln::MUL337 => 337u32,
            crate::pac::rcc::vals::Plln::MUL338 => 338u32,
            crate::pac::rcc::vals::Plln::MUL339 => 339u32,
            crate::pac::rcc::vals::Plln::MUL340 => 340u32,
            crate::pac::rcc::vals::Plln::MUL341 => 341u32,
            crate::pac::rcc::vals::Plln::MUL342 => 342u32,
            crate::pac::rcc::vals::Plln::MUL343 => 343u32,
            crate::pac::rcc::vals::Plln::MUL344 => 344u32,
            crate::pac::rcc::vals::Plln::MUL345 => 345u32,
            crate::pac::rcc::vals::Plln::MUL346 => 346u32,
            crate::pac::rcc::vals::Plln::MUL347 => 347u32,
            crate::pac::rcc::vals::Plln::MUL348 => 348u32,
            crate::pac::rcc::vals::Plln::MUL349 => 349u32,
            crate::pac::rcc::vals::Plln::MUL350 => 350u32,
            crate::pac::rcc::vals::Plln::MUL351 => 351u32,
            crate::pac::rcc::vals::Plln::MUL352 => 352u32,
            crate::pac::rcc::vals::Plln::MUL353 => 353u32,
            crate::pac::rcc::vals::Plln::MUL354 => 354u32,
            crate::pac::rcc::vals::Plln::MUL355 => 355u32,
            crate::pac::rcc::vals::Plln::MUL356 => 356u32,
            crate::pac::rcc::vals::Plln::MUL357 => 357u32,
            crate::pac::rcc::vals::Plln::MUL358 => 358u32,
            crate::pac::rcc::vals::Plln::MUL359 => 359u32,
            crate::pac::rcc::vals::Plln::MUL360 => 360u32,
            crate::pac::rcc::vals::Plln::MUL361 => 361u32,
            crate::pac::rcc::vals::Plln::MUL362 => 362u32,
            crate::pac::rcc::vals::Plln::MUL363 => 363u32,
            crate::pac::rcc::vals::Plln::MUL364 => 364u32,
            crate::pac::rcc::vals::Plln::MUL365 => 365u32,
            crate::pac::rcc::vals::Plln::MUL366 => 366u32,
            crate::pac::rcc::vals::Plln::MUL367 => 367u32,
            crate::pac::rcc::vals::Plln::MUL368 => 368u32,
            crate::pac::rcc::vals::Plln::MUL369 => 369u32,
            crate::pac::rcc::vals::Plln::MUL370 => 370u32,
            crate::pac::rcc::vals::Plln::MUL371 => 371u32,
            crate::pac::rcc::vals::Plln::MUL372 => 372u32,
            crate::pac::rcc::vals::Plln::MUL373 => 373u32,
            crate::pac::rcc::vals::Plln::MUL374 => 374u32,
            crate::pac::rcc::vals::Plln::MUL375 => 375u32,
            crate::pac::rcc::vals::Plln::MUL376 => 376u32,
            crate::pac::rcc::vals::Plln::MUL377 => 377u32,
            crate::pac::rcc::vals::Plln::MUL378 => 378u32,
            crate::pac::rcc::vals::Plln::MUL379 => 379u32,
            crate::pac::rcc::vals::Plln::MUL380 => 380u32,
            crate::pac::rcc::vals::Plln::MUL381 => 381u32,
            crate::pac::rcc::vals::Plln::MUL382 => 382u32,
            crate::pac::rcc::vals::Plln::MUL383 => 383u32,
            crate::pac::rcc::vals::Plln::MUL384 => 384u32,
            crate::pac::rcc::vals::Plln::MUL385 => 385u32,
            crate::pac::rcc::vals::Plln::MUL386 => 386u32,
            crate::pac::rcc::vals::Plln::MUL387 => 387u32,
            crate::pac::rcc::vals::Plln::MUL388 => 388u32,
            crate::pac::rcc::vals::Plln::MUL389 => 389u32,
            crate::pac::rcc::vals::Plln::MUL390 => 390u32,
            crate::pac::rcc::vals::Plln::MUL391 => 391u32,
            crate::pac::rcc::vals::Plln::MUL392 => 392u32,
            crate::pac::rcc::vals::Plln::MUL393 => 393u32,
            crate::pac::rcc::vals::Plln::MUL394 => 394u32,
            crate::pac::rcc::vals::Plln::MUL395 => 395u32,
            crate::pac::rcc::vals::Plln::MUL396 => 396u32,
            crate::pac::rcc::vals::Plln::MUL397 => 397u32,
            crate::pac::rcc::vals::Plln::MUL398 => 398u32,
            crate::pac::rcc::vals::Plln::MUL399 => 399u32,
            crate::pac::rcc::vals::Plln::MUL400 => 400u32,
            crate::pac::rcc::vals::Plln::MUL401 => 401u32,
            crate::pac::rcc::vals::Plln::MUL402 => 402u32,
            crate::pac::rcc::vals::Plln::MUL403 => 403u32,
            crate::pac::rcc::vals::Plln::MUL404 => 404u32,
            crate::pac::rcc::vals::Plln::MUL405 => 405u32,
            crate::pac::rcc::vals::Plln::MUL406 => 406u32,
            crate::pac::rcc::vals::Plln::MUL407 => 407u32,
            crate::pac::rcc::vals::Plln::MUL408 => 408u32,
            crate::pac::rcc::vals::Plln::MUL409 => 409u32,
            crate::pac::rcc::vals::Plln::MUL410 => 410u32,
            crate::pac::rcc::vals::Plln::MUL411 => 411u32,
            crate::pac::rcc::vals::Plln::MUL412 => 412u32,
            crate::pac::rcc::vals::Plln::MUL413 => 413u32,
            crate::pac::rcc::vals::Plln::MUL414 => 414u32,
            crate::pac::rcc::vals::Plln::MUL415 => 415u32,
            crate::pac::rcc::vals::Plln::MUL416 => 416u32,
            crate::pac::rcc::vals::Plln::MUL417 => 417u32,
            crate::pac::rcc::vals::Plln::MUL418 => 418u32,
            crate::pac::rcc::vals::Plln::MUL419 => 419u32,
            crate::pac::rcc::vals::Plln::MUL420 => 420u32,
            crate::pac::rcc::vals::Plln::MUL421 => 421u32,
            crate::pac::rcc::vals::Plln::MUL422 => 422u32,
            crate::pac::rcc::vals::Plln::MUL423 => 423u32,
            crate::pac::rcc::vals::Plln::MUL424 => 424u32,
            crate::pac::rcc::vals::Plln::MUL425 => 425u32,
            crate::pac::rcc::vals::Plln::MUL426 => 426u32,
            crate::pac::rcc::vals::Plln::MUL427 => 427u32,
            crate::pac::rcc::vals::Plln::MUL428 => 428u32,
            crate::pac::rcc::vals::Plln::MUL429 => 429u32,
            crate::pac::rcc::vals::Plln::MUL430 => 430u32,
            crate::pac::rcc::vals::Plln::MUL431 => 431u32,
            crate::pac::rcc::vals::Plln::MUL432 => 432u32,
            crate::pac::rcc::vals::Plln::MUL433 => 433u32,
            crate::pac::rcc::vals::Plln::MUL434 => 434u32,
            crate::pac::rcc::vals::Plln::MUL435 => 435u32,
            crate::pac::rcc::vals::Plln::MUL436 => 436u32,
            crate::pac::rcc::vals::Plln::MUL437 => 437u32,
            crate::pac::rcc::vals::Plln::MUL438 => 438u32,
            crate::pac::rcc::vals::Plln::MUL439 => 439u32,
            crate::pac::rcc::vals::Plln::MUL440 => 440u32,
            crate::pac::rcc::vals::Plln::MUL441 => 441u32,
            crate::pac::rcc::vals::Plln::MUL442 => 442u32,
            crate::pac::rcc::vals::Plln::MUL443 => 443u32,
            crate::pac::rcc::vals::Plln::MUL444 => 444u32,
            crate::pac::rcc::vals::Plln::MUL445 => 445u32,
            crate::pac::rcc::vals::Plln::MUL446 => 446u32,
            crate::pac::rcc::vals::Plln::MUL447 => 447u32,
            crate::pac::rcc::vals::Plln::MUL448 => 448u32,
            crate::pac::rcc::vals::Plln::MUL449 => 449u32,
            crate::pac::rcc::vals::Plln::MUL450 => 450u32,
            crate::pac::rcc::vals::Plln::MUL451 => 451u32,
            crate::pac::rcc::vals::Plln::MUL452 => 452u32,
            crate::pac::rcc::vals::Plln::MUL453 => 453u32,
            crate::pac::rcc::vals::Plln::MUL454 => 454u32,
            crate::pac::rcc::vals::Plln::MUL455 => 455u32,
            crate::pac::rcc::vals::Plln::MUL456 => 456u32,
            crate::pac::rcc::vals::Plln::MUL457 => 457u32,
            crate::pac::rcc::vals::Plln::MUL458 => 458u32,
            crate::pac::rcc::vals::Plln::MUL459 => 459u32,
            crate::pac::rcc::vals::Plln::MUL460 => 460u32,
            crate::pac::rcc::vals::Plln::MUL461 => 461u32,
            crate::pac::rcc::vals::Plln::MUL462 => 462u32,
            crate::pac::rcc::vals::Plln::MUL463 => 463u32,
            crate::pac::rcc::vals::Plln::MUL464 => 464u32,
            crate::pac::rcc::vals::Plln::MUL465 => 465u32,
            crate::pac::rcc::vals::Plln::MUL466 => 466u32,
            crate::pac::rcc::vals::Plln::MUL467 => 467u32,
            crate::pac::rcc::vals::Plln::MUL468 => 468u32,
            crate::pac::rcc::vals::Plln::MUL469 => 469u32,
            crate::pac::rcc::vals::Plln::MUL470 => 470u32,
            crate::pac::rcc::vals::Plln::MUL471 => 471u32,
            crate::pac::rcc::vals::Plln::MUL472 => 472u32,
            crate::pac::rcc::vals::Plln::MUL473 => 473u32,
            crate::pac::rcc::vals::Plln::MUL474 => 474u32,
            crate::pac::rcc::vals::Plln::MUL475 => 475u32,
            crate::pac::rcc::vals::Plln::MUL476 => 476u32,
            crate::pac::rcc::vals::Plln::MUL477 => 477u32,
            crate::pac::rcc::vals::Plln::MUL478 => 478u32,
            crate::pac::rcc::vals::Plln::MUL479 => 479u32,
            crate::pac::rcc::vals::Plln::MUL480 => 480u32,
            crate::pac::rcc::vals::Plln::MUL481 => 481u32,
            crate::pac::rcc::vals::Plln::MUL482 => 482u32,
            crate::pac::rcc::vals::Plln::MUL483 => 483u32,
            crate::pac::rcc::vals::Plln::MUL484 => 484u32,
            crate::pac::rcc::vals::Plln::MUL485 => 485u32,
            crate::pac::rcc::vals::Plln::MUL486 => 486u32,
            crate::pac::rcc::vals::Plln::MUL487 => 487u32,
            crate::pac::rcc::vals::Plln::MUL488 => 488u32,
            crate::pac::rcc::vals::Plln::MUL489 => 489u32,
            crate::pac::rcc::vals::Plln::MUL490 => 490u32,
            crate::pac::rcc::vals::Plln::MUL491 => 491u32,
            crate::pac::rcc::vals::Plln::MUL492 => 492u32,
            crate::pac::rcc::vals::Plln::MUL493 => 493u32,
            crate::pac::rcc::vals::Plln::MUL494 => 494u32,
            crate::pac::rcc::vals::Plln::MUL495 => 495u32,
            crate::pac::rcc::vals::Plln::MUL496 => 496u32,
            crate::pac::rcc::vals::Plln::MUL497 => 497u32,
            crate::pac::rcc::vals::Plln::MUL498 => 498u32,
            crate::pac::rcc::vals::Plln::MUL499 => 499u32,
            crate::pac::rcc::vals::Plln::MUL500 => 500u32,
            crate::pac::rcc::vals::Plln::MUL501 => 501u32,
            crate::pac::rcc::vals::Plln::MUL502 => 502u32,
            crate::pac::rcc::vals::Plln::MUL503 => 503u32,
            crate::pac::rcc::vals::Plln::MUL504 => 504u32,
            crate::pac::rcc::vals::Plln::MUL505 => 505u32,
            crate::pac::rcc::vals::Plln::MUL506 => 506u32,
            crate::pac::rcc::vals::Plln::MUL507 => 507u32,
            crate::pac::rcc::vals::Plln::MUL508 => 508u32,
            crate::pac::rcc::vals::Plln::MUL509 => 509u32,
            crate::pac::rcc::vals::Plln::MUL510 => 510u32,
            crate::pac::rcc::vals::Plln::MUL511 => 511u32,
            crate::pac::rcc::vals::Plln::MUL512 => 512u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Plln::MUL4 => 1u32,
            crate::pac::rcc::vals::Plln::MUL5 => 1u32,
            crate::pac::rcc::vals::Plln::MUL6 => 1u32,
            crate::pac::rcc::vals::Plln::MUL7 => 1u32,
            crate::pac::rcc::vals::Plln::MUL8 => 1u32,
            crate::pac::rcc::vals::Plln::MUL9 => 1u32,
            crate::pac::rcc::vals::Plln::MUL10 => 1u32,
            crate::pac::rcc::vals::Plln::MUL11 => 1u32,
            crate::pac::rcc::vals::Plln::MUL12 => 1u32,
            crate::pac::rcc::vals::Plln::MUL13 => 1u32,
            crate::pac::rcc::vals::Plln::MUL14 => 1u32,
            crate::pac::rcc::vals::Plln::MUL15 => 1u32,
            crate::pac::rcc::vals::Plln::MUL16 => 1u32,
            crate::pac::rcc::vals::Plln::MUL17 => 1u32,
            crate::pac::rcc::vals::Plln::MUL18 => 1u32,
            crate::pac::rcc::vals::Plln::MUL19 => 1u32,
            crate::pac::rcc::vals::Plln::MUL20 => 1u32,
            crate::pac::rcc::vals::Plln::MUL21 => 1u32,
            crate::pac::rcc::vals::Plln::MUL22 => 1u32,
            crate::pac::rcc::vals::Plln::MUL23 => 1u32,
            crate::pac::rcc::vals::Plln::MUL24 => 1u32,
            crate::pac::rcc::vals::Plln::MUL25 => 1u32,
            crate::pac::rcc::vals::Plln::MUL26 => 1u32,
            crate::pac::rcc::vals::Plln::MUL27 => 1u32,
            crate::pac::rcc::vals::Plln::MUL28 => 1u32,
            crate::pac::rcc::vals::Plln::MUL29 => 1u32,
            crate::pac::rcc::vals::Plln::MUL30 => 1u32,
            crate::pac::rcc::vals::Plln::MUL31 => 1u32,
            crate::pac::rcc::vals::Plln::MUL32 => 1u32,
            crate::pac::rcc::vals::Plln::MUL33 => 1u32,
            crate::pac::rcc::vals::Plln::MUL34 => 1u32,
            crate::pac::rcc::vals::Plln::MUL35 => 1u32,
            crate::pac::rcc::vals::Plln::MUL36 => 1u32,
            crate::pac::rcc::vals::Plln::MUL37 => 1u32,
            crate::pac::rcc::vals::Plln::MUL38 => 1u32,
            crate::pac::rcc::vals::Plln::MUL39 => 1u32,
            crate::pac::rcc::vals::Plln::MUL40 => 1u32,
            crate::pac::rcc::vals::Plln::MUL41 => 1u32,
            crate::pac::rcc::vals::Plln::MUL42 => 1u32,
            crate::pac::rcc::vals::Plln::MUL43 => 1u32,
            crate::pac::rcc::vals::Plln::MUL44 => 1u32,
            crate::pac::rcc::vals::Plln::MUL45 => 1u32,
            crate::pac::rcc::vals::Plln::MUL46 => 1u32,
            crate::pac::rcc::vals::Plln::MUL47 => 1u32,
            crate::pac::rcc::vals::Plln::MUL48 => 1u32,
            crate::pac::rcc::vals::Plln::MUL49 => 1u32,
            crate::pac::rcc::vals::Plln::MUL50 => 1u32,
            crate::pac::rcc::vals::Plln::MUL51 => 1u32,
            crate::pac::rcc::vals::Plln::MUL52 => 1u32,
            crate::pac::rcc::vals::Plln::MUL53 => 1u32,
            crate::pac::rcc::vals::Plln::MUL54 => 1u32,
            crate::pac::rcc::vals::Plln::MUL55 => 1u32,
            crate::pac::rcc::vals::Plln::MUL56 => 1u32,
            crate::pac::rcc::vals::Plln::MUL57 => 1u32,
            crate::pac::rcc::vals::Plln::MUL58 => 1u32,
            crate::pac::rcc::vals::Plln::MUL59 => 1u32,
            crate::pac::rcc::vals::Plln::MUL60 => 1u32,
            crate::pac::rcc::vals::Plln::MUL61 => 1u32,
            crate::pac::rcc::vals::Plln::MUL62 => 1u32,
            crate::pac::rcc::vals::Plln::MUL63 => 1u32,
            crate::pac::rcc::vals::Plln::MUL64 => 1u32,
            crate::pac::rcc::vals::Plln::MUL65 => 1u32,
            crate::pac::rcc::vals::Plln::MUL66 => 1u32,
            crate::pac::rcc::vals::Plln::MUL67 => 1u32,
            crate::pac::rcc::vals::Plln::MUL68 => 1u32,
            crate::pac::rcc::vals::Plln::MUL69 => 1u32,
            crate::pac::rcc::vals::Plln::MUL70 => 1u32,
            crate::pac::rcc::vals::Plln::MUL71 => 1u32,
            crate::pac::rcc::vals::Plln::MUL72 => 1u32,
            crate::pac::rcc::vals::Plln::MUL73 => 1u32,
            crate::pac::rcc::vals::Plln::MUL74 => 1u32,
            crate::pac::rcc::vals::Plln::MUL75 => 1u32,
            crate::pac::rcc::vals::Plln::MUL76 => 1u32,
            crate::pac::rcc::vals::Plln::MUL77 => 1u32,
            crate::pac::rcc::vals::Plln::MUL78 => 1u32,
            crate::pac::rcc::vals::Plln::MUL79 => 1u32,
            crate::pac::rcc::vals::Plln::MUL80 => 1u32,
            crate::pac::rcc::vals::Plln::MUL81 => 1u32,
            crate::pac::rcc::vals::Plln::MUL82 => 1u32,
            crate::pac::rcc::vals::Plln::MUL83 => 1u32,
            crate::pac::rcc::vals::Plln::MUL84 => 1u32,
            crate::pac::rcc::vals::Plln::MUL85 => 1u32,
            crate::pac::rcc::vals::Plln::MUL86 => 1u32,
            crate::pac::rcc::vals::Plln::MUL87 => 1u32,
            crate::pac::rcc::vals::Plln::MUL88 => 1u32,
            crate::pac::rcc::vals::Plln::MUL89 => 1u32,
            crate::pac::rcc::vals::Plln::MUL90 => 1u32,
            crate::pac::rcc::vals::Plln::MUL91 => 1u32,
            crate::pac::rcc::vals::Plln::MUL92 => 1u32,
            crate::pac::rcc::vals::Plln::MUL93 => 1u32,
            crate::pac::rcc::vals::Plln::MUL94 => 1u32,
            crate::pac::rcc::vals::Plln::MUL95 => 1u32,
            crate::pac::rcc::vals::Plln::MUL96 => 1u32,
            crate::pac::rcc::vals::Plln::MUL97 => 1u32,
            crate::pac::rcc::vals::Plln::MUL98 => 1u32,
            crate::pac::rcc::vals::Plln::MUL99 => 1u32,
            crate::pac::rcc::vals::Plln::MUL100 => 1u32,
            crate::pac::rcc::vals::Plln::MUL101 => 1u32,
            crate::pac::rcc::vals::Plln::MUL102 => 1u32,
            crate::pac::rcc::vals::Plln::MUL103 => 1u32,
            crate::pac::rcc::vals::Plln::MUL104 => 1u32,
            crate::pac::rcc::vals::Plln::MUL105 => 1u32,
            crate::pac::rcc::vals::Plln::MUL106 => 1u32,
            crate::pac::rcc::vals::Plln::MUL107 => 1u32,
            crate::pac::rcc::vals::Plln::MUL108 => 1u32,
            crate::pac::rcc::vals::Plln::MUL109 => 1u32,
            crate::pac::rcc::vals::Plln::MUL110 => 1u32,
            crate::pac::rcc::vals::Plln::MUL111 => 1u32,
            crate::pac::rcc::vals::Plln::MUL112 => 1u32,
            crate::pac::rcc::vals::Plln::MUL113 => 1u32,
            crate::pac::rcc::vals::Plln::MUL114 => 1u32,
            crate::pac::rcc::vals::Plln::MUL115 => 1u32,
            crate::pac::rcc::vals::Plln::MUL116 => 1u32,
            crate::pac::rcc::vals::Plln::MUL117 => 1u32,
            crate::pac::rcc::vals::Plln::MUL118 => 1u32,
            crate::pac::rcc::vals::Plln::MUL119 => 1u32,
            crate::pac::rcc::vals::Plln::MUL120 => 1u32,
            crate::pac::rcc::vals::Plln::MUL121 => 1u32,
            crate::pac::rcc::vals::Plln::MUL122 => 1u32,
            crate::pac::rcc::vals::Plln::MUL123 => 1u32,
            crate::pac::rcc::vals::Plln::MUL124 => 1u32,
            crate::pac::rcc::vals::Plln::MUL125 => 1u32,
            crate::pac::rcc::vals::Plln::MUL126 => 1u32,
            crate::pac::rcc::vals::Plln::MUL127 => 1u32,
            crate::pac::rcc::vals::Plln::MUL128 => 1u32,
            crate::pac::rcc::vals::Plln::MUL129 => 1u32,
            crate::pac::rcc::vals::Plln::MUL130 => 1u32,
            crate::pac::rcc::vals::Plln::MUL131 => 1u32,
            crate::pac::rcc::vals::Plln::MUL132 => 1u32,
            crate::pac::rcc::vals::Plln::MUL133 => 1u32,
            crate::pac::rcc::vals::Plln::MUL134 => 1u32,
            crate::pac::rcc::vals::Plln::MUL135 => 1u32,
            crate::pac::rcc::vals::Plln::MUL136 => 1u32,
            crate::pac::rcc::vals::Plln::MUL137 => 1u32,
            crate::pac::rcc::vals::Plln::MUL138 => 1u32,
            crate::pac::rcc::vals::Plln::MUL139 => 1u32,
            crate::pac::rcc::vals::Plln::MUL140 => 1u32,
            crate::pac::rcc::vals::Plln::MUL141 => 1u32,
            crate::pac::rcc::vals::Plln::MUL142 => 1u32,
            crate::pac::rcc::vals::Plln::MUL143 => 1u32,
            crate::pac::rcc::vals::Plln::MUL144 => 1u32,
            crate::pac::rcc::vals::Plln::MUL145 => 1u32,
            crate::pac::rcc::vals::Plln::MUL146 => 1u32,
            crate::pac::rcc::vals::Plln::MUL147 => 1u32,
            crate::pac::rcc::vals::Plln::MUL148 => 1u32,
            crate::pac::rcc::vals::Plln::MUL149 => 1u32,
            crate::pac::rcc::vals::Plln::MUL150 => 1u32,
            crate::pac::rcc::vals::Plln::MUL151 => 1u32,
            crate::pac::rcc::vals::Plln::MUL152 => 1u32,
            crate::pac::rcc::vals::Plln::MUL153 => 1u32,
            crate::pac::rcc::vals::Plln::MUL154 => 1u32,
            crate::pac::rcc::vals::Plln::MUL155 => 1u32,
            crate::pac::rcc::vals::Plln::MUL156 => 1u32,
            crate::pac::rcc::vals::Plln::MUL157 => 1u32,
            crate::pac::rcc::vals::Plln::MUL158 => 1u32,
            crate::pac::rcc::vals::Plln::MUL159 => 1u32,
            crate::pac::rcc::vals::Plln::MUL160 => 1u32,
            crate::pac::rcc::vals::Plln::MUL161 => 1u32,
            crate::pac::rcc::vals::Plln::MUL162 => 1u32,
            crate::pac::rcc::vals::Plln::MUL163 => 1u32,
            crate::pac::rcc::vals::Plln::MUL164 => 1u32,
            crate::pac::rcc::vals::Plln::MUL165 => 1u32,
            crate::pac::rcc::vals::Plln::MUL166 => 1u32,
            crate::pac::rcc::vals::Plln::MUL167 => 1u32,
            crate::pac::rcc::vals::Plln::MUL168 => 1u32,
            crate::pac::rcc::vals::Plln::MUL169 => 1u32,
            crate::pac::rcc::vals::Plln::MUL170 => 1u32,
            crate::pac::rcc::vals::Plln::MUL171 => 1u32,
            crate::pac::rcc::vals::Plln::MUL172 => 1u32,
            crate::pac::rcc::vals::Plln::MUL173 => 1u32,
            crate::pac::rcc::vals::Plln::MUL174 => 1u32,
            crate::pac::rcc::vals::Plln::MUL175 => 1u32,
            crate::pac::rcc::vals::Plln::MUL176 => 1u32,
            crate::pac::rcc::vals::Plln::MUL177 => 1u32,
            crate::pac::rcc::vals::Plln::MUL178 => 1u32,
            crate::pac::rcc::vals::Plln::MUL179 => 1u32,
            crate::pac::rcc::vals::Plln::MUL180 => 1u32,
            crate::pac::rcc::vals::Plln::MUL181 => 1u32,
            crate::pac::rcc::vals::Plln::MUL182 => 1u32,
            crate::pac::rcc::vals::Plln::MUL183 => 1u32,
            crate::pac::rcc::vals::Plln::MUL184 => 1u32,
            crate::pac::rcc::vals::Plln::MUL185 => 1u32,
            crate::pac::rcc::vals::Plln::MUL186 => 1u32,
            crate::pac::rcc::vals::Plln::MUL187 => 1u32,
            crate::pac::rcc::vals::Plln::MUL188 => 1u32,
            crate::pac::rcc::vals::Plln::MUL189 => 1u32,
            crate::pac::rcc::vals::Plln::MUL190 => 1u32,
            crate::pac::rcc::vals::Plln::MUL191 => 1u32,
            crate::pac::rcc::vals::Plln::MUL192 => 1u32,
            crate::pac::rcc::vals::Plln::MUL193 => 1u32,
            crate::pac::rcc::vals::Plln::MUL194 => 1u32,
            crate::pac::rcc::vals::Plln::MUL195 => 1u32,
            crate::pac::rcc::vals::Plln::MUL196 => 1u32,
            crate::pac::rcc::vals::Plln::MUL197 => 1u32,
            crate::pac::rcc::vals::Plln::MUL198 => 1u32,
            crate::pac::rcc::vals::Plln::MUL199 => 1u32,
            crate::pac::rcc::vals::Plln::MUL200 => 1u32,
            crate::pac::rcc::vals::Plln::MUL201 => 1u32,
            crate::pac::rcc::vals::Plln::MUL202 => 1u32,
            crate::pac::rcc::vals::Plln::MUL203 => 1u32,
            crate::pac::rcc::vals::Plln::MUL204 => 1u32,
            crate::pac::rcc::vals::Plln::MUL205 => 1u32,
            crate::pac::rcc::vals::Plln::MUL206 => 1u32,
            crate::pac::rcc::vals::Plln::MUL207 => 1u32,
            crate::pac::rcc::vals::Plln::MUL208 => 1u32,
            crate::pac::rcc::vals::Plln::MUL209 => 1u32,
            crate::pac::rcc::vals::Plln::MUL210 => 1u32,
            crate::pac::rcc::vals::Plln::MUL211 => 1u32,
            crate::pac::rcc::vals::Plln::MUL212 => 1u32,
            crate::pac::rcc::vals::Plln::MUL213 => 1u32,
            crate::pac::rcc::vals::Plln::MUL214 => 1u32,
            crate::pac::rcc::vals::Plln::MUL215 => 1u32,
            crate::pac::rcc::vals::Plln::MUL216 => 1u32,
            crate::pac::rcc::vals::Plln::MUL217 => 1u32,
            crate::pac::rcc::vals::Plln::MUL218 => 1u32,
            crate::pac::rcc::vals::Plln::MUL219 => 1u32,
            crate::pac::rcc::vals::Plln::MUL220 => 1u32,
            crate::pac::rcc::vals::Plln::MUL221 => 1u32,
            crate::pac::rcc::vals::Plln::MUL222 => 1u32,
            crate::pac::rcc::vals::Plln::MUL223 => 1u32,
            crate::pac::rcc::vals::Plln::MUL224 => 1u32,
            crate::pac::rcc::vals::Plln::MUL225 => 1u32,
            crate::pac::rcc::vals::Plln::MUL226 => 1u32,
            crate::pac::rcc::vals::Plln::MUL227 => 1u32,
            crate::pac::rcc::vals::Plln::MUL228 => 1u32,
            crate::pac::rcc::vals::Plln::MUL229 => 1u32,
            crate::pac::rcc::vals::Plln::MUL230 => 1u32,
            crate::pac::rcc::vals::Plln::MUL231 => 1u32,
            crate::pac::rcc::vals::Plln::MUL232 => 1u32,
            crate::pac::rcc::vals::Plln::MUL233 => 1u32,
            crate::pac::rcc::vals::Plln::MUL234 => 1u32,
            crate::pac::rcc::vals::Plln::MUL235 => 1u32,
            crate::pac::rcc::vals::Plln::MUL236 => 1u32,
            crate::pac::rcc::vals::Plln::MUL237 => 1u32,
            crate::pac::rcc::vals::Plln::MUL238 => 1u32,
            crate::pac::rcc::vals::Plln::MUL239 => 1u32,
            crate::pac::rcc::vals::Plln::MUL240 => 1u32,
            crate::pac::rcc::vals::Plln::MUL241 => 1u32,
            crate::pac::rcc::vals::Plln::MUL242 => 1u32,
            crate::pac::rcc::vals::Plln::MUL243 => 1u32,
            crate::pac::rcc::vals::Plln::MUL244 => 1u32,
            crate::pac::rcc::vals::Plln::MUL245 => 1u32,
            crate::pac::rcc::vals::Plln::MUL246 => 1u32,
            crate::pac::rcc::vals::Plln::MUL247 => 1u32,
            crate::pac::rcc::vals::Plln::MUL248 => 1u32,
            crate::pac::rcc::vals::Plln::MUL249 => 1u32,
            crate::pac::rcc::vals::Plln::MUL250 => 1u32,
            crate::pac::rcc::vals::Plln::MUL251 => 1u32,
            crate::pac::rcc::vals::Plln::MUL252 => 1u32,
            crate::pac::rcc::vals::Plln::MUL253 => 1u32,
            crate::pac::rcc::vals::Plln::MUL254 => 1u32,
            crate::pac::rcc::vals::Plln::MUL255 => 1u32,
            crate::pac::rcc::vals::Plln::MUL256 => 1u32,
            crate::pac::rcc::vals::Plln::MUL257 => 1u32,
            crate::pac::rcc::vals::Plln::MUL258 => 1u32,
            crate::pac::rcc::vals::Plln::MUL259 => 1u32,
            crate::pac::rcc::vals::Plln::MUL260 => 1u32,
            crate::pac::rcc::vals::Plln::MUL261 => 1u32,
            crate::pac::rcc::vals::Plln::MUL262 => 1u32,
            crate::pac::rcc::vals::Plln::MUL263 => 1u32,
            crate::pac::rcc::vals::Plln::MUL264 => 1u32,
            crate::pac::rcc::vals::Plln::MUL265 => 1u32,
            crate::pac::rcc::vals::Plln::MUL266 => 1u32,
            crate::pac::rcc::vals::Plln::MUL267 => 1u32,
            crate::pac::rcc::vals::Plln::MUL268 => 1u32,
            crate::pac::rcc::vals::Plln::MUL269 => 1u32,
            crate::pac::rcc::vals::Plln::MUL270 => 1u32,
            crate::pac::rcc::vals::Plln::MUL271 => 1u32,
            crate::pac::rcc::vals::Plln::MUL272 => 1u32,
            crate::pac::rcc::vals::Plln::MUL273 => 1u32,
            crate::pac::rcc::vals::Plln::MUL274 => 1u32,
            crate::pac::rcc::vals::Plln::MUL275 => 1u32,
            crate::pac::rcc::vals::Plln::MUL276 => 1u32,
            crate::pac::rcc::vals::Plln::MUL277 => 1u32,
            crate::pac::rcc::vals::Plln::MUL278 => 1u32,
            crate::pac::rcc::vals::Plln::MUL279 => 1u32,
            crate::pac::rcc::vals::Plln::MUL280 => 1u32,
            crate::pac::rcc::vals::Plln::MUL281 => 1u32,
            crate::pac::rcc::vals::Plln::MUL282 => 1u32,
            crate::pac::rcc::vals::Plln::MUL283 => 1u32,
            crate::pac::rcc::vals::Plln::MUL284 => 1u32,
            crate::pac::rcc::vals::Plln::MUL285 => 1u32,
            crate::pac::rcc::vals::Plln::MUL286 => 1u32,
            crate::pac::rcc::vals::Plln::MUL287 => 1u32,
            crate::pac::rcc::vals::Plln::MUL288 => 1u32,
            crate::pac::rcc::vals::Plln::MUL289 => 1u32,
            crate::pac::rcc::vals::Plln::MUL290 => 1u32,
            crate::pac::rcc::vals::Plln::MUL291 => 1u32,
            crate::pac::rcc::vals::Plln::MUL292 => 1u32,
            crate::pac::rcc::vals::Plln::MUL293 => 1u32,
            crate::pac::rcc::vals::Plln::MUL294 => 1u32,
            crate::pac::rcc::vals::Plln::MUL295 => 1u32,
            crate::pac::rcc::vals::Plln::MUL296 => 1u32,
            crate::pac::rcc::vals::Plln::MUL297 => 1u32,
            crate::pac::rcc::vals::Plln::MUL298 => 1u32,
            crate::pac::rcc::vals::Plln::MUL299 => 1u32,
            crate::pac::rcc::vals::Plln::MUL300 => 1u32,
            crate::pac::rcc::vals::Plln::MUL301 => 1u32,
            crate::pac::rcc::vals::Plln::MUL302 => 1u32,
            crate::pac::rcc::vals::Plln::MUL303 => 1u32,
            crate::pac::rcc::vals::Plln::MUL304 => 1u32,
            crate::pac::rcc::vals::Plln::MUL305 => 1u32,
            crate::pac::rcc::vals::Plln::MUL306 => 1u32,
            crate::pac::rcc::vals::Plln::MUL307 => 1u32,
            crate::pac::rcc::vals::Plln::MUL308 => 1u32,
            crate::pac::rcc::vals::Plln::MUL309 => 1u32,
            crate::pac::rcc::vals::Plln::MUL310 => 1u32,
            crate::pac::rcc::vals::Plln::MUL311 => 1u32,
            crate::pac::rcc::vals::Plln::MUL312 => 1u32,
            crate::pac::rcc::vals::Plln::MUL313 => 1u32,
            crate::pac::rcc::vals::Plln::MUL314 => 1u32,
            crate::pac::rcc::vals::Plln::MUL315 => 1u32,
            crate::pac::rcc::vals::Plln::MUL316 => 1u32,
            crate::pac::rcc::vals::Plln::MUL317 => 1u32,
            crate::pac::rcc::vals::Plln::MUL318 => 1u32,
            crate::pac::rcc::vals::Plln::MUL319 => 1u32,
            crate::pac::rcc::vals::Plln::MUL320 => 1u32,
            crate::pac::rcc::vals::Plln::MUL321 => 1u32,
            crate::pac::rcc::vals::Plln::MUL322 => 1u32,
            crate::pac::rcc::vals::Plln::MUL323 => 1u32,
            crate::pac::rcc::vals::Plln::MUL324 => 1u32,
            crate::pac::rcc::vals::Plln::MUL325 => 1u32,
            crate::pac::rcc::vals::Plln::MUL326 => 1u32,
            crate::pac::rcc::vals::Plln::MUL327 => 1u32,
            crate::pac::rcc::vals::Plln::MUL328 => 1u32,
            crate::pac::rcc::vals::Plln::MUL329 => 1u32,
            crate::pac::rcc::vals::Plln::MUL330 => 1u32,
            crate::pac::rcc::vals::Plln::MUL331 => 1u32,
            crate::pac::rcc::vals::Plln::MUL332 => 1u32,
            crate::pac::rcc::vals::Plln::MUL333 => 1u32,
            crate::pac::rcc::vals::Plln::MUL334 => 1u32,
            crate::pac::rcc::vals::Plln::MUL335 => 1u32,
            crate::pac::rcc::vals::Plln::MUL336 => 1u32,
            crate::pac::rcc::vals::Plln::MUL337 => 1u32,
            crate::pac::rcc::vals::Plln::MUL338 => 1u32,
            crate::pac::rcc::vals::Plln::MUL339 => 1u32,
            crate::pac::rcc::vals::Plln::MUL340 => 1u32,
            crate::pac::rcc::vals::Plln::MUL341 => 1u32,
            crate::pac::rcc::vals::Plln::MUL342 => 1u32,
            crate::pac::rcc::vals::Plln::MUL343 => 1u32,
            crate::pac::rcc::vals::Plln::MUL344 => 1u32,
            crate::pac::rcc::vals::Plln::MUL345 => 1u32,
            crate::pac::rcc::vals::Plln::MUL346 => 1u32,
            crate::pac::rcc::vals::Plln::MUL347 => 1u32,
            crate::pac::rcc::vals::Plln::MUL348 => 1u32,
            crate::pac::rcc::vals::Plln::MUL349 => 1u32,
            crate::pac::rcc::vals::Plln::MUL350 => 1u32,
            crate::pac::rcc::vals::Plln::MUL351 => 1u32,
            crate::pac::rcc::vals::Plln::MUL352 => 1u32,
            crate::pac::rcc::vals::Plln::MUL353 => 1u32,
            crate::pac::rcc::vals::Plln::MUL354 => 1u32,
            crate::pac::rcc::vals::Plln::MUL355 => 1u32,
            crate::pac::rcc::vals::Plln::MUL356 => 1u32,
            crate::pac::rcc::vals::Plln::MUL357 => 1u32,
            crate::pac::rcc::vals::Plln::MUL358 => 1u32,
            crate::pac::rcc::vals::Plln::MUL359 => 1u32,
            crate::pac::rcc::vals::Plln::MUL360 => 1u32,
            crate::pac::rcc::vals::Plln::MUL361 => 1u32,
            crate::pac::rcc::vals::Plln::MUL362 => 1u32,
            crate::pac::rcc::vals::Plln::MUL363 => 1u32,
            crate::pac::rcc::vals::Plln::MUL364 => 1u32,
            crate::pac::rcc::vals::Plln::MUL365 => 1u32,
            crate::pac::rcc::vals::Plln::MUL366 => 1u32,
            crate::pac::rcc::vals::Plln::MUL367 => 1u32,
            crate::pac::rcc::vals::Plln::MUL368 => 1u32,
            crate::pac::rcc::vals::Plln::MUL369 => 1u32,
            crate::pac::rcc::vals::Plln::MUL370 => 1u32,
            crate::pac::rcc::vals::Plln::MUL371 => 1u32,
            crate::pac::rcc::vals::Plln::MUL372 => 1u32,
            crate::pac::rcc::vals::Plln::MUL373 => 1u32,
            crate::pac::rcc::vals::Plln::MUL374 => 1u32,
            crate::pac::rcc::vals::Plln::MUL375 => 1u32,
            crate::pac::rcc::vals::Plln::MUL376 => 1u32,
            crate::pac::rcc::vals::Plln::MUL377 => 1u32,
            crate::pac::rcc::vals::Plln::MUL378 => 1u32,
            crate::pac::rcc::vals::Plln::MUL379 => 1u32,
            crate::pac::rcc::vals::Plln::MUL380 => 1u32,
            crate::pac::rcc::vals::Plln::MUL381 => 1u32,
            crate::pac::rcc::vals::Plln::MUL382 => 1u32,
            crate::pac::rcc::vals::Plln::MUL383 => 1u32,
            crate::pac::rcc::vals::Plln::MUL384 => 1u32,
            crate::pac::rcc::vals::Plln::MUL385 => 1u32,
            crate::pac::rcc::vals::Plln::MUL386 => 1u32,
            crate::pac::rcc::vals::Plln::MUL387 => 1u32,
            crate::pac::rcc::vals::Plln::MUL388 => 1u32,
            crate::pac::rcc::vals::Plln::MUL389 => 1u32,
            crate::pac::rcc::vals::Plln::MUL390 => 1u32,
            crate::pac::rcc::vals::Plln::MUL391 => 1u32,
            crate::pac::rcc::vals::Plln::MUL392 => 1u32,
            crate::pac::rcc::vals::Plln::MUL393 => 1u32,
            crate::pac::rcc::vals::Plln::MUL394 => 1u32,
            crate::pac::rcc::vals::Plln::MUL395 => 1u32,
            crate::pac::rcc::vals::Plln::MUL396 => 1u32,
            crate::pac::rcc::vals::Plln::MUL397 => 1u32,
            crate::pac::rcc::vals::Plln::MUL398 => 1u32,
            crate::pac::rcc::vals::Plln::MUL399 => 1u32,
            crate::pac::rcc::vals::Plln::MUL400 => 1u32,
            crate::pac::rcc::vals::Plln::MUL401 => 1u32,
            crate::pac::rcc::vals::Plln::MUL402 => 1u32,
            crate::pac::rcc::vals::Plln::MUL403 => 1u32,
            crate::pac::rcc::vals::Plln::MUL404 => 1u32,
            crate::pac::rcc::vals::Plln::MUL405 => 1u32,
            crate::pac::rcc::vals::Plln::MUL406 => 1u32,
            crate::pac::rcc::vals::Plln::MUL407 => 1u32,
            crate::pac::rcc::vals::Plln::MUL408 => 1u32,
            crate::pac::rcc::vals::Plln::MUL409 => 1u32,
            crate::pac::rcc::vals::Plln::MUL410 => 1u32,
            crate::pac::rcc::vals::Plln::MUL411 => 1u32,
            crate::pac::rcc::vals::Plln::MUL412 => 1u32,
            crate::pac::rcc::vals::Plln::MUL413 => 1u32,
            crate::pac::rcc::vals::Plln::MUL414 => 1u32,
            crate::pac::rcc::vals::Plln::MUL415 => 1u32,
            crate::pac::rcc::vals::Plln::MUL416 => 1u32,
            crate::pac::rcc::vals::Plln::MUL417 => 1u32,
            crate::pac::rcc::vals::Plln::MUL418 => 1u32,
            crate::pac::rcc::vals::Plln::MUL419 => 1u32,
            crate::pac::rcc::vals::Plln::MUL420 => 1u32,
            crate::pac::rcc::vals::Plln::MUL421 => 1u32,
            crate::pac::rcc::vals::Plln::MUL422 => 1u32,
            crate::pac::rcc::vals::Plln::MUL423 => 1u32,
            crate::pac::rcc::vals::Plln::MUL424 => 1u32,
            crate::pac::rcc::vals::Plln::MUL425 => 1u32,
            crate::pac::rcc::vals::Plln::MUL426 => 1u32,
            crate::pac::rcc::vals::Plln::MUL427 => 1u32,
            crate::pac::rcc::vals::Plln::MUL428 => 1u32,
            crate::pac::rcc::vals::Plln::MUL429 => 1u32,
            crate::pac::rcc::vals::Plln::MUL430 => 1u32,
            crate::pac::rcc::vals::Plln::MUL431 => 1u32,
            crate::pac::rcc::vals::Plln::MUL432 => 1u32,
            crate::pac::rcc::vals::Plln::MUL433 => 1u32,
            crate::pac::rcc::vals::Plln::MUL434 => 1u32,
            crate::pac::rcc::vals::Plln::MUL435 => 1u32,
            crate::pac::rcc::vals::Plln::MUL436 => 1u32,
            crate::pac::rcc::vals::Plln::MUL437 => 1u32,
            crate::pac::rcc::vals::Plln::MUL438 => 1u32,
            crate::pac::rcc::vals::Plln::MUL439 => 1u32,
            crate::pac::rcc::vals::Plln::MUL440 => 1u32,
            crate::pac::rcc::vals::Plln::MUL441 => 1u32,
            crate::pac::rcc::vals::Plln::MUL442 => 1u32,
            crate::pac::rcc::vals::Plln::MUL443 => 1u32,
            crate::pac::rcc::vals::Plln::MUL444 => 1u32,
            crate::pac::rcc::vals::Plln::MUL445 => 1u32,
            crate::pac::rcc::vals::Plln::MUL446 => 1u32,
            crate::pac::rcc::vals::Plln::MUL447 => 1u32,
            crate::pac::rcc::vals::Plln::MUL448 => 1u32,
            crate::pac::rcc::vals::Plln::MUL449 => 1u32,
            crate::pac::rcc::vals::Plln::MUL450 => 1u32,
            crate::pac::rcc::vals::Plln::MUL451 => 1u32,
            crate::pac::rcc::vals::Plln::MUL452 => 1u32,
            crate::pac::rcc::vals::Plln::MUL453 => 1u32,
            crate::pac::rcc::vals::Plln::MUL454 => 1u32,
            crate::pac::rcc::vals::Plln::MUL455 => 1u32,
            crate::pac::rcc::vals::Plln::MUL456 => 1u32,
            crate::pac::rcc::vals::Plln::MUL457 => 1u32,
            crate::pac::rcc::vals::Plln::MUL458 => 1u32,
            crate::pac::rcc::vals::Plln::MUL459 => 1u32,
            crate::pac::rcc::vals::Plln::MUL460 => 1u32,
            crate::pac::rcc::vals::Plln::MUL461 => 1u32,
            crate::pac::rcc::vals::Plln::MUL462 => 1u32,
            crate::pac::rcc::vals::Plln::MUL463 => 1u32,
            crate::pac::rcc::vals::Plln::MUL464 => 1u32,
            crate::pac::rcc::vals::Plln::MUL465 => 1u32,
            crate::pac::rcc::vals::Plln::MUL466 => 1u32,
            crate::pac::rcc::vals::Plln::MUL467 => 1u32,
            crate::pac::rcc::vals::Plln::MUL468 => 1u32,
            crate::pac::rcc::vals::Plln::MUL469 => 1u32,
            crate::pac::rcc::vals::Plln::MUL470 => 1u32,
            crate::pac::rcc::vals::Plln::MUL471 => 1u32,
            crate::pac::rcc::vals::Plln::MUL472 => 1u32,
            crate::pac::rcc::vals::Plln::MUL473 => 1u32,
            crate::pac::rcc::vals::Plln::MUL474 => 1u32,
            crate::pac::rcc::vals::Plln::MUL475 => 1u32,
            crate::pac::rcc::vals::Plln::MUL476 => 1u32,
            crate::pac::rcc::vals::Plln::MUL477 => 1u32,
            crate::pac::rcc::vals::Plln::MUL478 => 1u32,
            crate::pac::rcc::vals::Plln::MUL479 => 1u32,
            crate::pac::rcc::vals::Plln::MUL480 => 1u32,
            crate::pac::rcc::vals::Plln::MUL481 => 1u32,
            crate::pac::rcc::vals::Plln::MUL482 => 1u32,
            crate::pac::rcc::vals::Plln::MUL483 => 1u32,
            crate::pac::rcc::vals::Plln::MUL484 => 1u32,
            crate::pac::rcc::vals::Plln::MUL485 => 1u32,
            crate::pac::rcc::vals::Plln::MUL486 => 1u32,
            crate::pac::rcc::vals::Plln::MUL487 => 1u32,
            crate::pac::rcc::vals::Plln::MUL488 => 1u32,
            crate::pac::rcc::vals::Plln::MUL489 => 1u32,
            crate::pac::rcc::vals::Plln::MUL490 => 1u32,
            crate::pac::rcc::vals::Plln::MUL491 => 1u32,
            crate::pac::rcc::vals::Plln::MUL492 => 1u32,
            crate::pac::rcc::vals::Plln::MUL493 => 1u32,
            crate::pac::rcc::vals::Plln::MUL494 => 1u32,
            crate::pac::rcc::vals::Plln::MUL495 => 1u32,
            crate::pac::rcc::vals::Plln::MUL496 => 1u32,
            crate::pac::rcc::vals::Plln::MUL497 => 1u32,
            crate::pac::rcc::vals::Plln::MUL498 => 1u32,
            crate::pac::rcc::vals::Plln::MUL499 => 1u32,
            crate::pac::rcc::vals::Plln::MUL500 => 1u32,
            crate::pac::rcc::vals::Plln::MUL501 => 1u32,
            crate::pac::rcc::vals::Plln::MUL502 => 1u32,
            crate::pac::rcc::vals::Plln::MUL503 => 1u32,
            crate::pac::rcc::vals::Plln::MUL504 => 1u32,
            crate::pac::rcc::vals::Plln::MUL505 => 1u32,
            crate::pac::rcc::vals::Plln::MUL506 => 1u32,
            crate::pac::rcc::vals::Plln::MUL507 => 1u32,
            crate::pac::rcc::vals::Plln::MUL508 => 1u32,
            crate::pac::rcc::vals::Plln::MUL509 => 1u32,
            crate::pac::rcc::vals::Plln::MUL510 => 1u32,
            crate::pac::rcc::vals::Plln::MUL511 => 1u32,
            crate::pac::rcc::vals::Plln::MUL512 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl crate::time::Prescaler for crate::pac::rcc::vals::Ppre {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Ppre::DIV1 => 1u32,
            crate::pac::rcc::vals::Ppre::DIV2 => 2u32,
            crate::pac::rcc::vals::Ppre::DIV4 => 4u32,
            crate::pac::rcc::vals::Ppre::DIV8 => 8u32,
            crate::pac::rcc::vals::Ppre::DIV16 => 16u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::rcc::vals::Ppre::DIV1 => 1u32,
            crate::pac::rcc::vals::Ppre::DIV2 => 1u32,
            crate::pac::rcc::vals::Ppre::DIV4 => 1u32,
            crate::pac::rcc::vals::Ppre::DIV8 => 1u32,
            crate::pac::rcc::vals::Ppre::DIV16 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl crate::time::Prescaler for crate::pac::adccommon::vals::Presc {
    fn num(&self) -> u32 {
        match *self {
            crate::pac::adccommon::vals::Presc::DIV1 => 1u32,
            crate::pac::adccommon::vals::Presc::DIV2 => 2u32,
            crate::pac::adccommon::vals::Presc::DIV4 => 4u32,
            crate::pac::adccommon::vals::Presc::DIV6 => 6u32,
            crate::pac::adccommon::vals::Presc::DIV8 => 8u32,
            crate::pac::adccommon::vals::Presc::DIV10 => 10u32,
            crate::pac::adccommon::vals::Presc::DIV12 => 12u32,
            crate::pac::adccommon::vals::Presc::DIV16 => 16u32,
            crate::pac::adccommon::vals::Presc::DIV32 => 32u32,
            crate::pac::adccommon::vals::Presc::DIV64 => 64u32,
            crate::pac::adccommon::vals::Presc::DIV128 => 128u32,
            crate::pac::adccommon::vals::Presc::DIV256 => 256u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
    fn denom(&self) -> u32 {
        match *self {
            crate::pac::adccommon::vals::Presc::DIV1 => 1u32,
            crate::pac::adccommon::vals::Presc::DIV2 => 1u32,
            crate::pac::adccommon::vals::Presc::DIV4 => 1u32,
            crate::pac::adccommon::vals::Presc::DIV6 => 1u32,
            crate::pac::adccommon::vals::Presc::DIV8 => 1u32,
            crate::pac::adccommon::vals::Presc::DIV10 => 1u32,
            crate::pac::adccommon::vals::Presc::DIV12 => 1u32,
            crate::pac::adccommon::vals::Presc::DIV16 => 1u32,
            crate::pac::adccommon::vals::Presc::DIV32 => 1u32,
            crate::pac::adccommon::vals::Presc::DIV64 => 1u32,
            crate::pac::adccommon::vals::Presc::DIV128 => 1u32,
            crate::pac::adccommon::vals::Presc::DIV256 => 1u32,
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
#[allow(non_camel_case_types)]
pub mod peripheral_interrupts {
    pub mod ADC1 {
        pub type GLOBAL = crate::interrupt::typelevel::ADC;
    }
    pub mod ADC12_COMMON {}
    pub mod ADC2 {
        pub type GLOBAL = crate::interrupt::typelevel::ADC;
    }
    pub mod ADC3 {
        pub type GLOBAL = crate::interrupt::typelevel::ADC3;
    }
    pub mod ADC3_COMMON {}
    pub mod BDMA {
        pub type CH0 = crate::interrupt::typelevel::BDMA_CHANNEL0;
        pub type CH1 = crate::interrupt::typelevel::BDMA_CHANNEL1;
        pub type CH2 = crate::interrupt::typelevel::BDMA_CHANNEL2;
        pub type CH3 = crate::interrupt::typelevel::BDMA_CHANNEL3;
        pub type CH4 = crate::interrupt::typelevel::BDMA_CHANNEL4;
        pub type CH5 = crate::interrupt::typelevel::BDMA_CHANNEL5;
        pub type CH6 = crate::interrupt::typelevel::BDMA_CHANNEL6;
        pub type CH7 = crate::interrupt::typelevel::BDMA_CHANNEL7;
    }
    pub mod CEC {
        pub type GLOBAL = crate::interrupt::typelevel::CEC;
    }
    pub mod COMP1 {
        pub type WKUP = crate::interrupt::typelevel::COMP;
    }
    pub mod COMP2 {
        pub type WKUP = crate::interrupt::typelevel::COMP;
    }
    pub mod CRC {}
    pub mod CRS {}
    pub mod DAC1 {
        pub type GLOBAL = crate::interrupt::typelevel::TIM6_DAC;
    }
    pub mod DBGMCU {}
    pub mod DCMI {
        pub type GLOBAL = crate::interrupt::typelevel::DCMI;
    }
    pub mod DFSDM1 {
        pub type FLT0 = crate::interrupt::typelevel::DFSDM1_FLT0;
        pub type FLT1 = crate::interrupt::typelevel::DFSDM1_FLT1;
        pub type FLT2 = crate::interrupt::typelevel::DFSDM1_FLT2;
        pub type FLT3 = crate::interrupt::typelevel::DFSDM1_FLT3;
    }
    pub mod DLYB_QUADSPI {}
    pub mod DLYB_SDMMC1 {}
    pub mod DLYB_SDMMC2 {}
    pub mod DMA1 {
        pub type CH0 = crate::interrupt::typelevel::DMA1_STREAM0;
        pub type CH1 = crate::interrupt::typelevel::DMA1_STREAM1;
        pub type CH2 = crate::interrupt::typelevel::DMA1_STREAM2;
        pub type CH3 = crate::interrupt::typelevel::DMA1_STREAM3;
        pub type CH4 = crate::interrupt::typelevel::DMA1_STREAM4;
        pub type CH5 = crate::interrupt::typelevel::DMA1_STREAM5;
        pub type CH6 = crate::interrupt::typelevel::DMA1_STREAM6;
        pub type CH7 = crate::interrupt::typelevel::DMA1_STREAM7;
    }
    pub mod DMA2 {
        pub type CH0 = crate::interrupt::typelevel::DMA2_STREAM0;
        pub type CH1 = crate::interrupt::typelevel::DMA2_STREAM1;
        pub type CH2 = crate::interrupt::typelevel::DMA2_STREAM2;
        pub type CH3 = crate::interrupt::typelevel::DMA2_STREAM3;
        pub type CH4 = crate::interrupt::typelevel::DMA2_STREAM4;
        pub type CH5 = crate::interrupt::typelevel::DMA2_STREAM5;
        pub type CH6 = crate::interrupt::typelevel::DMA2_STREAM6;
        pub type CH7 = crate::interrupt::typelevel::DMA2_STREAM7;
    }
    pub mod DMA2D {
        pub type GLOBAL = crate::interrupt::typelevel::DMA2D;
    }
    pub mod DMAMUX1 {
        pub type OVR = crate::interrupt::typelevel::DMAMUX1_OVR;
    }
    pub mod DMAMUX2 {
        pub type OVR = crate::interrupt::typelevel::DMAMUX2_OVR;
    }
    pub mod ETH {
        pub type GLOBAL = crate::interrupt::typelevel::ETH;
        pub type WKUP = crate::interrupt::typelevel::ETH_WKUP;
    }
    pub mod EXTI {
        pub type EXTI0 = crate::interrupt::typelevel::EXTI0;
        pub type EXTI1 = crate::interrupt::typelevel::EXTI1;
        pub type EXTI10 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI11 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI12 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI13 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI14 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI15 = crate::interrupt::typelevel::EXTI15_10;
        pub type EXTI2 = crate::interrupt::typelevel::EXTI2;
        pub type EXTI3 = crate::interrupt::typelevel::EXTI3;
        pub type EXTI4 = crate::interrupt::typelevel::EXTI4;
        pub type EXTI5 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI6 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI7 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI8 = crate::interrupt::typelevel::EXTI9_5;
        pub type EXTI9 = crate::interrupt::typelevel::EXTI9_5;
    }
    pub mod FDCAN1 {
        pub type CAL = crate::interrupt::typelevel::FDCAN_CAL;
        pub type IT0 = crate::interrupt::typelevel::FDCAN1_IT0;
        pub type IT1 = crate::interrupt::typelevel::FDCAN1_IT1;
    }
    pub mod FDCAN2 {
        pub type CAL = crate::interrupt::typelevel::FDCAN_CAL;
        pub type IT0 = crate::interrupt::typelevel::FDCAN2_IT0;
        pub type IT1 = crate::interrupt::typelevel::FDCAN2_IT1;
    }
    pub mod FDCANRAM {}
    pub mod FLASH {
        pub type GLOBAL = crate::interrupt::typelevel::FLASH;
    }
    pub mod FMC {
        pub type GLOBAL = crate::interrupt::typelevel::FMC;
    }
    pub mod GPIOA {}
    pub mod GPIOB {}
    pub mod GPIOC {}
    pub mod GPIOD {}
    pub mod GPIOE {}
    pub mod GPIOF {}
    pub mod GPIOG {}
    pub mod GPIOH {}
    pub mod GPIOI {}
    pub mod GPIOJ {}
    pub mod GPIOK {}
    pub mod HRTIM1 {
        pub type FLT = crate::interrupt::typelevel::HRTIM1_FLT;
        pub type MASTER = crate::interrupt::typelevel::HRTIM1_MASTER;
        pub type TIMA = crate::interrupt::typelevel::HRTIM1_TIMA;
        pub type TIMB = crate::interrupt::typelevel::HRTIM1_TIMB;
        pub type TIMC = crate::interrupt::typelevel::HRTIM1_TIMC;
        pub type TIMD = crate::interrupt::typelevel::HRTIM1_TIMD;
        pub type TIME = crate::interrupt::typelevel::HRTIM1_TIME;
    }
    pub mod HSEM {
        pub type GLOBAL = crate::interrupt::typelevel::HSEM1;
    }
    pub mod I2C1 {
        pub type ER = crate::interrupt::typelevel::I2C1_ER;
        pub type EV = crate::interrupt::typelevel::I2C1_EV;
    }
    pub mod I2C2 {
        pub type ER = crate::interrupt::typelevel::I2C2_ER;
        pub type EV = crate::interrupt::typelevel::I2C2_EV;
    }
    pub mod I2C3 {
        pub type ER = crate::interrupt::typelevel::I2C3_ER;
        pub type EV = crate::interrupt::typelevel::I2C3_EV;
    }
    pub mod I2C4 {
        pub type ER = crate::interrupt::typelevel::I2C4_ER;
        pub type EV = crate::interrupt::typelevel::I2C4_EV;
    }
    pub mod IWDG1 {}
    pub mod JPEG {
        pub type GLOBAL = crate::interrupt::typelevel::JPEG;
    }
    pub mod LPTIM1 {
        pub type GLOBAL = crate::interrupt::typelevel::LPTIM1;
    }
    pub mod LPTIM2 {
        pub type GLOBAL = crate::interrupt::typelevel::LPTIM2;
    }
    pub mod LPTIM3 {
        pub type GLOBAL = crate::interrupt::typelevel::LPTIM3;
    }
    pub mod LPTIM4 {
        pub type GLOBAL = crate::interrupt::typelevel::LPTIM4;
    }
    pub mod LPTIM5 {
        pub type GLOBAL = crate::interrupt::typelevel::LPTIM5;
    }
    pub mod LPUART1 {
        pub type GLOBAL = crate::interrupt::typelevel::LPUART1;
    }
    pub mod LTDC {
        pub type ER = crate::interrupt::typelevel::LTDC_ER;
        pub type LO = crate::interrupt::typelevel::LTDC;
    }
    pub mod MDIOS {
        pub type GLOBAL = crate::interrupt::typelevel::MDIOS;
        pub type WKUP = crate::interrupt::typelevel::MDIOS_WKUP;
    }
    pub mod MDMA {
        pub type GLOBAL = crate::interrupt::typelevel::MDMA;
    }
    pub mod OPAMP1 {}
    pub mod OPAMP2 {}
    pub mod PWR {}
    pub mod QUADSPI {
        pub type GLOBAL = crate::interrupt::typelevel::QUADSPI;
    }
    pub mod RCC {
        pub type CRS = crate::interrupt::typelevel::CRS;
        pub type GLOBAL = crate::interrupt::typelevel::RCC;
    }
    pub mod RNG {
        pub type GLOBAL = crate::interrupt::typelevel::RNG;
    }
    pub mod RTC {
        pub type ALARM = crate::interrupt::typelevel::RTC_ALARM;
        pub type STAMP = crate::interrupt::typelevel::TAMP_STAMP;
        pub type TAMP = crate::interrupt::typelevel::TAMP_STAMP;
        pub type WKUP = crate::interrupt::typelevel::RTC_WKUP;
    }
    pub mod SAI1 {
        pub type A = crate::interrupt::typelevel::SAI1;
        pub type B = crate::interrupt::typelevel::SAI1;
    }
    pub mod SAI2 {
        pub type A = crate::interrupt::typelevel::SAI2;
        pub type B = crate::interrupt::typelevel::SAI2;
    }
    pub mod SAI3 {
        pub type A = crate::interrupt::typelevel::SAI3;
        pub type B = crate::interrupt::typelevel::SAI3;
    }
    pub mod SAI4 {
        pub type A = crate::interrupt::typelevel::SAI4;
        pub type B = crate::interrupt::typelevel::SAI4;
    }
    pub mod SDMMC1 {
        pub type GLOBAL = crate::interrupt::typelevel::SDMMC1;
    }
    pub mod SDMMC2 {
        pub type GLOBAL = crate::interrupt::typelevel::SDMMC2;
    }
    pub mod SPDIFRX1 {
        pub type GLOBAL = crate::interrupt::typelevel::SPDIF_RX;
    }
    pub mod SPI1 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI1;
    }
    pub mod SPI2 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI2;
    }
    pub mod SPI3 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI3;
    }
    pub mod SPI4 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI4;
    }
    pub mod SPI6 {
        pub type GLOBAL = crate::interrupt::typelevel::SPI6;
    }
    pub mod SWPMI1 {
        pub type GLOBAL = crate::interrupt::typelevel::SWPMI1;
    }
    pub mod SYSCFG {}
    pub mod TIM1 {
        pub type BRK = crate::interrupt::typelevel::TIM1_BRK;
        pub type CC = crate::interrupt::typelevel::TIM1_CC;
        pub type COM = crate::interrupt::typelevel::TIM1_TRG_COM;
        pub type TRG = crate::interrupt::typelevel::TIM1_TRG_COM;
        pub type UP = crate::interrupt::typelevel::TIM1_UP;
    }
    pub mod TIM12 {
        pub type BRK = crate::interrupt::typelevel::TIM8_BRK_TIM12;
        pub type CC = crate::interrupt::typelevel::TIM8_BRK_TIM12;
        pub type COM = crate::interrupt::typelevel::TIM8_BRK_TIM12;
        pub type TRG = crate::interrupt::typelevel::TIM8_BRK_TIM12;
        pub type UP = crate::interrupt::typelevel::TIM8_BRK_TIM12;
    }
    pub mod TIM13 {
        pub type BRK = crate::interrupt::typelevel::TIM8_UP_TIM13;
        pub type CC = crate::interrupt::typelevel::TIM8_UP_TIM13;
        pub type COM = crate::interrupt::typelevel::TIM8_UP_TIM13;
        pub type TRG = crate::interrupt::typelevel::TIM8_UP_TIM13;
        pub type UP = crate::interrupt::typelevel::TIM8_UP_TIM13;
    }
    pub mod TIM14 {
        pub type BRK = crate::interrupt::typelevel::TIM8_TRG_COM_TIM14;
        pub type CC = crate::interrupt::typelevel::TIM8_TRG_COM_TIM14;
        pub type COM = crate::interrupt::typelevel::TIM8_TRG_COM_TIM14;
        pub type TRG = crate::interrupt::typelevel::TIM8_TRG_COM_TIM14;
        pub type UP = crate::interrupt::typelevel::TIM8_TRG_COM_TIM14;
    }
    pub mod TIM15 {
        pub type BRK = crate::interrupt::typelevel::TIM15;
        pub type CC = crate::interrupt::typelevel::TIM15;
        pub type COM = crate::interrupt::typelevel::TIM15;
        pub type TRG = crate::interrupt::typelevel::TIM15;
        pub type UP = crate::interrupt::typelevel::TIM15;
    }
    pub mod TIM16 {
        pub type BRK = crate::interrupt::typelevel::TIM16;
        pub type CC = crate::interrupt::typelevel::TIM16;
        pub type COM = crate::interrupt::typelevel::TIM16;
        pub type TRG = crate::interrupt::typelevel::TIM16;
        pub type UP = crate::interrupt::typelevel::TIM16;
    }
    pub mod TIM17 {
        pub type BRK = crate::interrupt::typelevel::TIM17;
        pub type CC = crate::interrupt::typelevel::TIM17;
        pub type COM = crate::interrupt::typelevel::TIM17;
        pub type TRG = crate::interrupt::typelevel::TIM17;
        pub type UP = crate::interrupt::typelevel::TIM17;
    }
    pub mod TIM2 {
        pub type BRK = crate::interrupt::typelevel::TIM2;
        pub type CC = crate::interrupt::typelevel::TIM2;
        pub type COM = crate::interrupt::typelevel::TIM2;
        pub type TRG = crate::interrupt::typelevel::TIM2;
        pub type UP = crate::interrupt::typelevel::TIM2;
    }
    pub mod TIM3 {
        pub type BRK = crate::interrupt::typelevel::TIM3;
        pub type CC = crate::interrupt::typelevel::TIM3;
        pub type COM = crate::interrupt::typelevel::TIM3;
        pub type TRG = crate::interrupt::typelevel::TIM3;
        pub type UP = crate::interrupt::typelevel::TIM3;
    }
    pub mod TIM4 {
        pub type BRK = crate::interrupt::typelevel::TIM4;
        pub type CC = crate::interrupt::typelevel::TIM4;
        pub type COM = crate::interrupt::typelevel::TIM4;
        pub type TRG = crate::interrupt::typelevel::TIM4;
        pub type UP = crate::interrupt::typelevel::TIM4;
    }
    pub mod TIM5 {
        pub type BRK = crate::interrupt::typelevel::TIM5;
        pub type CC = crate::interrupt::typelevel::TIM5;
        pub type COM = crate::interrupt::typelevel::TIM5;
        pub type TRG = crate::interrupt::typelevel::TIM5;
        pub type UP = crate::interrupt::typelevel::TIM5;
    }
    pub mod TIM6 {
        pub type BRK = crate::interrupt::typelevel::TIM6_DAC;
        pub type CC = crate::interrupt::typelevel::TIM6_DAC;
        pub type COM = crate::interrupt::typelevel::TIM6_DAC;
        pub type TRG = crate::interrupt::typelevel::TIM6_DAC;
        pub type UP = crate::interrupt::typelevel::TIM6_DAC;
    }
    pub mod TIM7 {
        pub type BRK = crate::interrupt::typelevel::TIM7;
        pub type CC = crate::interrupt::typelevel::TIM7;
        pub type COM = crate::interrupt::typelevel::TIM7;
        pub type TRG = crate::interrupt::typelevel::TIM7;
        pub type UP = crate::interrupt::typelevel::TIM7;
    }
    pub mod TIM8 {
        pub type BRK = crate::interrupt::typelevel::TIM8_BRK_TIM12;
        pub type CC = crate::interrupt::typelevel::TIM8_CC;
        pub type COM = crate::interrupt::typelevel::TIM8_TRG_COM_TIM14;
        pub type TRG = crate::interrupt::typelevel::TIM8_TRG_COM_TIM14;
        pub type UP = crate::interrupt::typelevel::TIM8_UP_TIM13;
    }
    pub mod UART4 {
        pub type GLOBAL = crate::interrupt::typelevel::UART4;
    }
    pub mod UART5 {
        pub type GLOBAL = crate::interrupt::typelevel::UART5;
    }
    pub mod UART7 {
        pub type GLOBAL = crate::interrupt::typelevel::UART7;
    }
    pub mod UART8 {
        pub type GLOBAL = crate::interrupt::typelevel::UART8;
    }
    pub mod UID {}
    pub mod USART1 {
        pub type GLOBAL = crate::interrupt::typelevel::USART1;
    }
    pub mod USART2 {
        pub type GLOBAL = crate::interrupt::typelevel::USART2;
    }
    pub mod USART3 {
        pub type GLOBAL = crate::interrupt::typelevel::USART3;
    }
    pub mod USART6 {
        pub type GLOBAL = crate::interrupt::typelevel::USART6;
    }
    pub mod USB_OTG_FS {
        pub type EP1_IN = crate::interrupt::typelevel::OTG_FS_EP1_IN;
        pub type EP1_OUT = crate::interrupt::typelevel::OTG_FS_EP1_OUT;
        pub type GLOBAL = crate::interrupt::typelevel::OTG_FS;
        pub type WKUP = crate::interrupt::typelevel::OTG_FS_WKUP;
    }
    pub mod USB_OTG_HS {
        pub type EP1_IN = crate::interrupt::typelevel::OTG_HS_EP1_IN;
        pub type EP1_OUT = crate::interrupt::typelevel::OTG_HS_EP1_OUT;
        pub type GLOBAL = crate::interrupt::typelevel::OTG_HS;
        pub type WKUP = crate::interrupt::typelevel::OTG_HS_WKUP;
    }
    pub mod VREFBUF {}
    pub mod WWDG1 {
        pub type GLOBAL = crate::interrupt::typelevel::WWDG;
        pub type RST = crate::interrupt::typelevel::WWDG;
    }
}
dma_channel_impl!(DMA1_CH0, 0u8, crate::interrupt::typelevel::DMA1_STREAM0);
dma_channel_impl!(DMA1_CH1, 1u8, crate::interrupt::typelevel::DMA1_STREAM1);
dma_channel_impl!(DMA1_CH2, 2u8, crate::interrupt::typelevel::DMA1_STREAM2);
dma_channel_impl!(DMA1_CH3, 3u8, crate::interrupt::typelevel::DMA1_STREAM3);
dma_channel_impl!(DMA1_CH4, 4u8, crate::interrupt::typelevel::DMA1_STREAM4);
dma_channel_impl!(DMA1_CH5, 5u8, crate::interrupt::typelevel::DMA1_STREAM5);
dma_channel_impl!(DMA1_CH6, 6u8, crate::interrupt::typelevel::DMA1_STREAM6);
dma_channel_impl!(DMA1_CH7, 7u8, crate::interrupt::typelevel::DMA1_STREAM7);
dma_channel_impl!(DMA2_CH0, 8u8, crate::interrupt::typelevel::DMA2_STREAM0);
dma_channel_impl!(DMA2_CH1, 9u8, crate::interrupt::typelevel::DMA2_STREAM1);
dma_channel_impl!(DMA2_CH2, 10u8, crate::interrupt::typelevel::DMA2_STREAM2);
dma_channel_impl!(DMA2_CH3, 11u8, crate::interrupt::typelevel::DMA2_STREAM3);
dma_channel_impl!(DMA2_CH4, 12u8, crate::interrupt::typelevel::DMA2_STREAM4);
dma_channel_impl!(DMA2_CH5, 13u8, crate::interrupt::typelevel::DMA2_STREAM5);
dma_channel_impl!(DMA2_CH6, 14u8, crate::interrupt::typelevel::DMA2_STREAM6);
dma_channel_impl!(DMA2_CH7, 15u8, crate::interrupt::typelevel::DMA2_STREAM7);
dma_channel_impl!(BDMA_CH0, 16u8, crate::interrupt::typelevel::BDMA_CHANNEL0);
dma_channel_impl!(BDMA_CH1, 17u8, crate::interrupt::typelevel::BDMA_CHANNEL1);
dma_channel_impl!(BDMA_CH2, 18u8, crate::interrupt::typelevel::BDMA_CHANNEL2);
dma_channel_impl!(BDMA_CH3, 19u8, crate::interrupt::typelevel::BDMA_CHANNEL3);
dma_channel_impl!(BDMA_CH4, 20u8, crate::interrupt::typelevel::BDMA_CHANNEL4);
dma_channel_impl!(BDMA_CH5, 21u8, crate::interrupt::typelevel::BDMA_CHANNEL5);
dma_channel_impl!(BDMA_CH6, 22u8, crate::interrupt::typelevel::BDMA_CHANNEL6);
dma_channel_impl!(BDMA_CH7, 23u8, crate::interrupt::typelevel::BDMA_CHANNEL7);
dma_channel_impl!(MDMA_CH0, 24u8, crate::interrupt::typelevel::MDMA);
dma_channel_impl!(MDMA_CH1, 25u8, crate::interrupt::typelevel::MDMA);
dma_channel_impl!(MDMA_CH2, 26u8, crate::interrupt::typelevel::MDMA);
dma_channel_impl!(MDMA_CH3, 27u8, crate::interrupt::typelevel::MDMA);
dma_channel_impl!(MDMA_CH4, 28u8, crate::interrupt::typelevel::MDMA);
dma_channel_impl!(MDMA_CH5, 29u8, crate::interrupt::typelevel::MDMA);
dma_channel_impl!(MDMA_CH6, 30u8, crate::interrupt::typelevel::MDMA);
dma_channel_impl!(MDMA_CH7, 31u8, crate::interrupt::typelevel::MDMA);
dma_channel_impl!(MDMA_CH8, 32u8, crate::interrupt::typelevel::MDMA);
dma_channel_impl!(MDMA_CH9, 33u8, crate::interrupt::typelevel::MDMA);
dma_channel_impl!(MDMA_CH10, 34u8, crate::interrupt::typelevel::MDMA);
dma_channel_impl!(MDMA_CH11, 35u8, crate::interrupt::typelevel::MDMA);
dma_channel_impl!(MDMA_CH12, 36u8, crate::interrupt::typelevel::MDMA);
dma_channel_impl!(MDMA_CH13, 37u8, crate::interrupt::typelevel::MDMA);
dma_channel_impl!(MDMA_CH14, 38u8, crate::interrupt::typelevel::MDMA);
dma_channel_impl!(MDMA_CH15, 39u8, crate::interrupt::typelevel::MDMA);
pub(crate) const DMA_CHANNELS: &[crate::dma::ChannelInfo] = &[
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 0usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 0usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 1usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 1usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 2usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 2usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 3usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 3usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 4usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 4usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 5usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 5usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 6usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 6usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA1),
        num: 7usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 7usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 0usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 8usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 1usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 9usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 2usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 10usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 3usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 11usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 4usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 12usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 5usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 13usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 6usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 14usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Dma(crate::pac::DMA2),
        num: 7usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX1,
            num: 15usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::BDMA),
        num: 0usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX2,
            num: 0usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::BDMA),
        num: 1usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX2,
            num: 1usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::BDMA),
        num: 2usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX2,
            num: 2usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::BDMA),
        num: 3usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX2,
            num: 3usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::BDMA),
        num: 4usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX2,
            num: 4usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::BDMA),
        num: 5usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX2,
            num: 5usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::BDMA),
        num: 6usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX2,
            num: 6usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Bdma(crate::pac::BDMA),
        num: 7usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: Some(crate::dma::DmamuxInfo {
            mux: crate::pac::DMAMUX2,
            num: 7usize,
        }),
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Mdma(crate::pac::MDMA),
        num: 0usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: None,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Mdma(crate::pac::MDMA),
        num: 1usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: None,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Mdma(crate::pac::MDMA),
        num: 2usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: None,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Mdma(crate::pac::MDMA),
        num: 3usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: None,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Mdma(crate::pac::MDMA),
        num: 4usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: None,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Mdma(crate::pac::MDMA),
        num: 5usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: None,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Mdma(crate::pac::MDMA),
        num: 6usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: None,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Mdma(crate::pac::MDMA),
        num: 7usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: None,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Mdma(crate::pac::MDMA),
        num: 8usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: None,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Mdma(crate::pac::MDMA),
        num: 9usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: None,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Mdma(crate::pac::MDMA),
        num: 10usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: None,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Mdma(crate::pac::MDMA),
        num: 11usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: None,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Mdma(crate::pac::MDMA),
        num: 12usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: None,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Mdma(crate::pac::MDMA),
        num: 13usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: None,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Mdma(crate::pac::MDMA),
        num: 14usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: None,
    },
    crate::dma::ChannelInfo {
        dma: crate::dma::DmaInfo::Mdma(crate::pac::MDMA),
        num: 15usize,
        #[cfg(feature = "low-power")]
        stop_mode: crate::rcc::StopMode::Stop1,
        dmamux: None,
    },
];
pub const fn gpio_block(port_num: usize) -> crate::pac::gpio::Gpio {
    #[cfg(stm32n6)]
    let port_num = if port_num > 7 { port_num + 5 } else { port_num };
    unsafe { crate::pac::gpio::Gpio::from_ptr((1476526080usize + 1024usize * port_num) as _) }
}
pub const MEMORY_REGION_ITCM: core::ops::Range<u32> = 0u32..65536u32;
pub const MEMORY_REGION_DTCM: core::ops::Range<u32> = 536870912u32..537001984u32;
pub const FLASH_BASE: usize = 134217728usize;
pub const FLASH_SIZE: usize = 2097152usize;
pub const WRITE_SIZE: usize = 32usize;
