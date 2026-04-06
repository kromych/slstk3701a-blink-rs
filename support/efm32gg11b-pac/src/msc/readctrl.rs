#[doc = "Register `READCTRL` reader"]
pub type R = crate::R<ReadctrlSpec>;
#[doc = "Register `READCTRL` writer"]
pub type W = crate::W<ReadctrlSpec>;
#[doc = "Field `IFCDIS` reader - Internal Flash Cache Disable"]
pub type IfcdisR = crate::BitReader;
#[doc = "Field `IFCDIS` writer - Internal Flash Cache Disable"]
pub type IfcdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIDIS` reader - Automatic Invalidate Disable"]
pub type AidisR = crate::BitReader;
#[doc = "Field `AIDIS` writer - Automatic Invalidate Disable"]
pub type AidisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICCDIS` reader - Interrupt Context Cache Disable"]
pub type IccdisR = crate::BitReader;
#[doc = "Field `ICCDIS` writer - Interrupt Context Cache Disable"]
pub type IccdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EBICDIS` reader - External Bus Interface Cache Disable"]
pub type EbicdisR = crate::BitReader;
#[doc = "Field `EBICDIS` writer - External Bus Interface Cache Disable"]
pub type EbicdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREFETCH` reader - Prefetch Mode"]
pub type PrefetchR = crate::BitReader;
#[doc = "Field `PREFETCH` writer - Prefetch Mode"]
pub type PrefetchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USEHPROT` reader - AHB_HPROT Mode"]
pub type UsehprotR = crate::BitReader;
#[doc = "Field `USEHPROT` writer - AHB_HPROT Mode"]
pub type UsehprotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPICDIS` reader - QSPI Cache Disable"]
pub type QspicdisR = crate::BitReader;
#[doc = "Field `QSPICDIS` writer - QSPI Cache Disable"]
pub type QspicdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Read Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Zero wait-states inserted in fetch or read transfers"]
    Ws0 = 0,
    #[doc = "1: One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details"]
    Ws1 = 1,
    #[doc = "2: Two wait-states inserted for eatch fetch or read transfer. See Flash Wait-States table for details"]
    Ws2 = 2,
    #[doc = "3: Three wait-states inserted for eatch fetch or read transfer. See Flash Wait-States table for details"]
    Ws3 = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Read Mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::Ws0,
            1 => Mode::Ws1,
            2 => Mode::Ws2,
            3 => Mode::Ws3,
            _ => unreachable!(),
        }
    }
    #[doc = "Zero wait-states inserted in fetch or read transfers"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == Mode::Ws0
    }
    #[doc = "One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == Mode::Ws1
    }
    #[doc = "Two wait-states inserted for eatch fetch or read transfer. See Flash Wait-States table for details"]
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == Mode::Ws2
    }
    #[doc = "Three wait-states inserted for eatch fetch or read transfer. See Flash Wait-States table for details"]
    #[inline(always)]
    pub fn is_ws3(&self) -> bool {
        *self == Mode::Ws3
    }
}
#[doc = "Field `MODE` writer - Read Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Zero wait-states inserted in fetch or read transfers"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ws0)
    }
    #[doc = "One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ws1)
    }
    #[doc = "Two wait-states inserted for eatch fetch or read transfer. See Flash Wait-States table for details"]
    #[inline(always)]
    pub fn ws2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ws2)
    }
    #[doc = "Three wait-states inserted for eatch fetch or read transfer. See Flash Wait-States table for details"]
    #[inline(always)]
    pub fn ws3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ws3)
    }
}
#[doc = "Field `SCBTP` reader - Suppress Conditional Branch Target Perfetch"]
pub type ScbtpR = crate::BitReader;
#[doc = "Field `SCBTP` writer - Suppress Conditional Branch Target Perfetch"]
pub type ScbtpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Internal Flash Cache Disable"]
    #[inline(always)]
    pub fn ifcdis(&self) -> IfcdisR {
        IfcdisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic Invalidate Disable"]
    #[inline(always)]
    pub fn aidis(&self) -> AidisR {
        AidisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Context Cache Disable"]
    #[inline(always)]
    pub fn iccdis(&self) -> IccdisR {
        IccdisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Bus Interface Cache Disable"]
    #[inline(always)]
    pub fn ebicdis(&self) -> EbicdisR {
        EbicdisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Prefetch Mode"]
    #[inline(always)]
    pub fn prefetch(&self) -> PrefetchR {
        PrefetchR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AHB_HPROT Mode"]
    #[inline(always)]
    pub fn usehprot(&self) -> UsehprotR {
        UsehprotR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - QSPI Cache Disable"]
    #[inline(always)]
    pub fn qspicdis(&self) -> QspicdisR {
        QspicdisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Read Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Suppress Conditional Branch Target Perfetch"]
    #[inline(always)]
    pub fn scbtp(&self) -> ScbtpR {
        ScbtpR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Internal Flash Cache Disable"]
    #[inline(always)]
    pub fn ifcdis(&mut self) -> IfcdisW<'_, ReadctrlSpec> {
        IfcdisW::new(self, 3)
    }
    #[doc = "Bit 4 - Automatic Invalidate Disable"]
    #[inline(always)]
    pub fn aidis(&mut self) -> AidisW<'_, ReadctrlSpec> {
        AidisW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt Context Cache Disable"]
    #[inline(always)]
    pub fn iccdis(&mut self) -> IccdisW<'_, ReadctrlSpec> {
        IccdisW::new(self, 5)
    }
    #[doc = "Bit 6 - External Bus Interface Cache Disable"]
    #[inline(always)]
    pub fn ebicdis(&mut self) -> EbicdisW<'_, ReadctrlSpec> {
        EbicdisW::new(self, 6)
    }
    #[doc = "Bit 8 - Prefetch Mode"]
    #[inline(always)]
    pub fn prefetch(&mut self) -> PrefetchW<'_, ReadctrlSpec> {
        PrefetchW::new(self, 8)
    }
    #[doc = "Bit 9 - AHB_HPROT Mode"]
    #[inline(always)]
    pub fn usehprot(&mut self) -> UsehprotW<'_, ReadctrlSpec> {
        UsehprotW::new(self, 9)
    }
    #[doc = "Bit 10 - QSPI Cache Disable"]
    #[inline(always)]
    pub fn qspicdis(&mut self) -> QspicdisW<'_, ReadctrlSpec> {
        QspicdisW::new(self, 10)
    }
    #[doc = "Bits 24:25 - Read Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, ReadctrlSpec> {
        ModeW::new(self, 24)
    }
    #[doc = "Bit 28 - Suppress Conditional Branch Target Perfetch"]
    #[inline(always)]
    pub fn scbtp(&mut self) -> ScbtpW<'_, ReadctrlSpec> {
        ScbtpW::new(self, 28)
    }
}
#[doc = "Read Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`readctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadctrlSpec;
impl crate::RegisterSpec for ReadctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`readctrl::R`](R) reader structure"]
impl crate::Readable for ReadctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`readctrl::W`](W) writer structure"]
impl crate::Writable for ReadctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets READCTRL to value 0x0100_0100"]
impl crate::Resettable for ReadctrlSpec {
    const RESET_VALUE: u32 = 0x0100_0100;
}
