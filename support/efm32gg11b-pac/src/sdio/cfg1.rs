#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "Field `ASYNCINTRSUP` reader - Asynchronous Interrupt Support"]
pub type AsyncintrsupR = crate::BitReader;
#[doc = "Field `ASYNCINTRSUP` writer - Asynchronous Interrupt Support"]
pub type AsyncintrsupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Slot Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Slottype {
    #[doc = "0: Removable SD Card Slot"]
    Rmsdslot = 0,
    #[doc = "1: Embedded SD Card Slot"]
    Emsdslot = 1,
    #[doc = "2: Shared SD Card Slot"]
    Shbusslot = 2,
}
impl From<Slottype> for u8 {
    #[inline(always)]
    fn from(variant: Slottype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Slottype {
    type Ux = u8;
}
impl crate::IsEnum for Slottype {}
#[doc = "Field `SLOTTYPE` reader - Slot Type"]
pub type SlottypeR = crate::FieldReader<Slottype>;
impl SlottypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Slottype> {
        match self.bits {
            0 => Some(Slottype::Rmsdslot),
            1 => Some(Slottype::Emsdslot),
            2 => Some(Slottype::Shbusslot),
            _ => None,
        }
    }
    #[doc = "Removable SD Card Slot"]
    #[inline(always)]
    pub fn is_rmsdslot(&self) -> bool {
        *self == Slottype::Rmsdslot
    }
    #[doc = "Embedded SD Card Slot"]
    #[inline(always)]
    pub fn is_emsdslot(&self) -> bool {
        *self == Slottype::Emsdslot
    }
    #[doc = "Shared SD Card Slot"]
    #[inline(always)]
    pub fn is_shbusslot(&self) -> bool {
        *self == Slottype::Shbusslot
    }
}
#[doc = "Field `SLOTTYPE` writer - Slot Type"]
pub type SlottypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Slottype>;
impl<'a, REG> SlottypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Removable SD Card Slot"]
    #[inline(always)]
    pub fn rmsdslot(self) -> &'a mut crate::W<REG> {
        self.variant(Slottype::Rmsdslot)
    }
    #[doc = "Embedded SD Card Slot"]
    #[inline(always)]
    pub fn emsdslot(self) -> &'a mut crate::W<REG> {
        self.variant(Slottype::Emsdslot)
    }
    #[doc = "Shared SD Card Slot"]
    #[inline(always)]
    pub fn shbusslot(self) -> &'a mut crate::W<REG> {
        self.variant(Slottype::Shbusslot)
    }
}
#[doc = "Field `CSDR50SUP` reader - Core Support SDR50"]
pub type Csdr50supR = crate::BitReader;
#[doc = "Field `CSDR50SUP` writer - Core Support SDR50"]
pub type Csdr50supW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSDR104SUP` reader - Support SDR104"]
pub type Csdr104supR = crate::BitReader;
#[doc = "Field `CSDR104SUP` writer - Support SDR104"]
pub type Csdr104supW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDDR50SUP` reader - Support DDR50"]
pub type Cddr50supR = crate::BitReader;
#[doc = "Field `CDDR50SUP` writer - Support DDR50"]
pub type Cddr50supW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDRVASUP` reader - Support Type a Driver"]
pub type CdrvasupR = crate::BitReader;
#[doc = "Field `CDRVASUP` writer - Support Type a Driver"]
pub type CdrvasupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDRVCSUP` reader - Support Type C Driver"]
pub type CdrvcsupR = crate::BitReader;
#[doc = "Field `CDRVCSUP` writer - Support Type C Driver"]
pub type CdrvcsupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDRVDSUP` reader - Support Type D Driver"]
pub type CdrvdsupR = crate::BitReader;
#[doc = "Field `CDRVDSUP` writer - Support Type D Driver"]
pub type CdrvdsupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETUNTMRCTL` reader - Retuning Timer Control"]
pub type RetuntmrctlR = crate::FieldReader;
#[doc = "Field `RETUNTMRCTL` writer - Retuning Timer Control"]
pub type RetuntmrctlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TUNSDR50` reader - Tuning for SDR50"]
pub type Tunsdr50R = crate::BitReader;
#[doc = "Field `TUNSDR50` writer - Tuning for SDR50"]
pub type Tunsdr50W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETUNMODES` reader - Retuning Modes"]
pub type RetunmodesR = crate::FieldReader;
#[doc = "Field `RETUNMODES` writer - Retuning Modes"]
pub type RetunmodesW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPISUP` reader - SPI Support"]
pub type SpisupR = crate::BitReader;
#[doc = "Field `SPISUP` writer - SPI Support"]
pub type SpisupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCWKUPEN` reader - Asynchronous Wakeup Enable"]
pub type AsyncwkupenR = crate::BitReader;
#[doc = "Field `ASYNCWKUPEN` writer - Asynchronous Wakeup Enable"]
pub type AsyncwkupenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Asynchronous Interrupt Support"]
    #[inline(always)]
    pub fn asyncintrsup(&self) -> AsyncintrsupR {
        AsyncintrsupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Slot Type"]
    #[inline(always)]
    pub fn slottype(&self) -> SlottypeR {
        SlottypeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Core Support SDR50"]
    #[inline(always)]
    pub fn csdr50sup(&self) -> Csdr50supR {
        Csdr50supR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Support SDR104"]
    #[inline(always)]
    pub fn csdr104sup(&self) -> Csdr104supR {
        Csdr104supR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Support DDR50"]
    #[inline(always)]
    pub fn cddr50sup(&self) -> Cddr50supR {
        Cddr50supR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Support Type a Driver"]
    #[inline(always)]
    pub fn cdrvasup(&self) -> CdrvasupR {
        CdrvasupR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Support Type C Driver"]
    #[inline(always)]
    pub fn cdrvcsup(&self) -> CdrvcsupR {
        CdrvcsupR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Support Type D Driver"]
    #[inline(always)]
    pub fn cdrvdsup(&self) -> CdrvdsupR {
        CdrvdsupR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:12 - Retuning Timer Control"]
    #[inline(always)]
    pub fn retuntmrctl(&self) -> RetuntmrctlR {
        RetuntmrctlR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Tuning for SDR50"]
    #[inline(always)]
    pub fn tunsdr50(&self) -> Tunsdr50R {
        Tunsdr50R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Retuning Modes"]
    #[inline(always)]
    pub fn retunmodes(&self) -> RetunmodesR {
        RetunmodesR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - SPI Support"]
    #[inline(always)]
    pub fn spisup(&self) -> SpisupR {
        SpisupR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Asynchronous Wakeup Enable"]
    #[inline(always)]
    pub fn asyncwkupen(&self) -> AsyncwkupenR {
        AsyncwkupenR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Asynchronous Interrupt Support"]
    #[inline(always)]
    pub fn asyncintrsup(&mut self) -> AsyncintrsupW<'_, Cfg1Spec> {
        AsyncintrsupW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Slot Type"]
    #[inline(always)]
    pub fn slottype(&mut self) -> SlottypeW<'_, Cfg1Spec> {
        SlottypeW::new(self, 1)
    }
    #[doc = "Bit 3 - Core Support SDR50"]
    #[inline(always)]
    pub fn csdr50sup(&mut self) -> Csdr50supW<'_, Cfg1Spec> {
        Csdr50supW::new(self, 3)
    }
    #[doc = "Bit 4 - Support SDR104"]
    #[inline(always)]
    pub fn csdr104sup(&mut self) -> Csdr104supW<'_, Cfg1Spec> {
        Csdr104supW::new(self, 4)
    }
    #[doc = "Bit 5 - Support DDR50"]
    #[inline(always)]
    pub fn cddr50sup(&mut self) -> Cddr50supW<'_, Cfg1Spec> {
        Cddr50supW::new(self, 5)
    }
    #[doc = "Bit 6 - Support Type a Driver"]
    #[inline(always)]
    pub fn cdrvasup(&mut self) -> CdrvasupW<'_, Cfg1Spec> {
        CdrvasupW::new(self, 6)
    }
    #[doc = "Bit 7 - Support Type C Driver"]
    #[inline(always)]
    pub fn cdrvcsup(&mut self) -> CdrvcsupW<'_, Cfg1Spec> {
        CdrvcsupW::new(self, 7)
    }
    #[doc = "Bit 8 - Support Type D Driver"]
    #[inline(always)]
    pub fn cdrvdsup(&mut self) -> CdrvdsupW<'_, Cfg1Spec> {
        CdrvdsupW::new(self, 8)
    }
    #[doc = "Bits 9:12 - Retuning Timer Control"]
    #[inline(always)]
    pub fn retuntmrctl(&mut self) -> RetuntmrctlW<'_, Cfg1Spec> {
        RetuntmrctlW::new(self, 9)
    }
    #[doc = "Bit 13 - Tuning for SDR50"]
    #[inline(always)]
    pub fn tunsdr50(&mut self) -> Tunsdr50W<'_, Cfg1Spec> {
        Tunsdr50W::new(self, 13)
    }
    #[doc = "Bits 14:15 - Retuning Modes"]
    #[inline(always)]
    pub fn retunmodes(&mut self) -> RetunmodesW<'_, Cfg1Spec> {
        RetunmodesW::new(self, 14)
    }
    #[doc = "Bit 16 - SPI Support"]
    #[inline(always)]
    pub fn spisup(&mut self) -> SpisupW<'_, Cfg1Spec> {
        SpisupW::new(self, 16)
    }
    #[doc = "Bit 18 - Asynchronous Wakeup Enable"]
    #[inline(always)]
    pub fn asyncwkupen(&mut self) -> AsyncwkupenW<'_, Cfg1Spec> {
        AsyncwkupenW::new(self, 18)
    }
}
#[doc = "Core Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for Cfg1Spec {}
