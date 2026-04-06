#[doc = "Register `HPRT` reader"]
pub type R = crate::R<HprtSpec>;
#[doc = "Register `HPRT` writer"]
pub type W = crate::W<HprtSpec>;
#[doc = "Field `PRTCONNSTS` reader - Port Connect Status"]
pub type PrtconnstsR = crate::BitReader;
#[doc = "Field `PRTCONNDET` reader - Port Connect Detected"]
pub type PrtconndetR = crate::BitReader;
#[doc = "Field `PRTCONNDET` writer - Port Connect Detected"]
pub type PrtconndetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTENA` reader - Port Enable"]
pub type PrtenaR = crate::BitReader;
#[doc = "Field `PRTENA` writer - Port Enable"]
pub type PrtenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTENCHNG` reader - Port Enable/Disable Change"]
pub type PrtenchngR = crate::BitReader;
#[doc = "Field `PRTENCHNG` writer - Port Enable/Disable Change"]
pub type PrtenchngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTOVRCURRACT` reader - Port Overcurrent Active"]
pub type PrtovrcurractR = crate::BitReader;
#[doc = "Field `PRTOVRCURRCHNG` reader - Port Overcurrent Change"]
pub type PrtovrcurrchngR = crate::BitReader;
#[doc = "Field `PRTOVRCURRCHNG` writer - Port Overcurrent Change"]
pub type PrtovrcurrchngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTRES` reader - Port Resume"]
pub type PrtresR = crate::BitReader;
#[doc = "Field `PRTRES` writer - Port Resume"]
pub type PrtresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTSUSP` reader - Port Suspend"]
pub type PrtsuspR = crate::BitReader;
#[doc = "Field `PRTSUSP` writer - Port Suspend"]
pub type PrtsuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTRST` reader - Port Reset"]
pub type PrtrstR = crate::BitReader;
#[doc = "Field `PRTRST` writer - Port Reset"]
pub type PrtrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTLNSTS` reader - Port Line Status"]
pub type PrtlnstsR = crate::FieldReader;
#[doc = "Field `PRTPWR` reader - Port Power"]
pub type PrtpwrR = crate::BitReader;
#[doc = "Field `PRTPWR` writer - Port Power"]
pub type PrtpwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Port Test Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prttstctl {
    #[doc = "0: Test mode disabled."]
    Disable = 0,
    #[doc = "1: Test_J mode."]
    J = 1,
    #[doc = "2: Test_K mode."]
    K = 2,
    #[doc = "3: Test_SE0_NAK mode."]
    Se0nak = 3,
    #[doc = "4: Test_Packet mode."]
    Packet = 4,
    #[doc = "5: Test_Force_Enable."]
    Force = 5,
}
impl From<Prttstctl> for u8 {
    #[inline(always)]
    fn from(variant: Prttstctl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prttstctl {
    type Ux = u8;
}
impl crate::IsEnum for Prttstctl {}
#[doc = "Field `PRTTSTCTL` reader - Port Test Control"]
pub type PrttstctlR = crate::FieldReader<Prttstctl>;
impl PrttstctlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prttstctl> {
        match self.bits {
            0 => Some(Prttstctl::Disable),
            1 => Some(Prttstctl::J),
            2 => Some(Prttstctl::K),
            3 => Some(Prttstctl::Se0nak),
            4 => Some(Prttstctl::Packet),
            5 => Some(Prttstctl::Force),
            _ => None,
        }
    }
    #[doc = "Test mode disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Prttstctl::Disable
    }
    #[doc = "Test_J mode."]
    #[inline(always)]
    pub fn is_j(&self) -> bool {
        *self == Prttstctl::J
    }
    #[doc = "Test_K mode."]
    #[inline(always)]
    pub fn is_k(&self) -> bool {
        *self == Prttstctl::K
    }
    #[doc = "Test_SE0_NAK mode."]
    #[inline(always)]
    pub fn is_se0nak(&self) -> bool {
        *self == Prttstctl::Se0nak
    }
    #[doc = "Test_Packet mode."]
    #[inline(always)]
    pub fn is_packet(&self) -> bool {
        *self == Prttstctl::Packet
    }
    #[doc = "Test_Force_Enable."]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == Prttstctl::Force
    }
}
#[doc = "Field `PRTTSTCTL` writer - Port Test Control"]
pub type PrttstctlW<'a, REG> = crate::FieldWriter<'a, REG, 4, Prttstctl>;
impl<'a, REG> PrttstctlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Test mode disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Prttstctl::Disable)
    }
    #[doc = "Test_J mode."]
    #[inline(always)]
    pub fn j(self) -> &'a mut crate::W<REG> {
        self.variant(Prttstctl::J)
    }
    #[doc = "Test_K mode."]
    #[inline(always)]
    pub fn k(self) -> &'a mut crate::W<REG> {
        self.variant(Prttstctl::K)
    }
    #[doc = "Test_SE0_NAK mode."]
    #[inline(always)]
    pub fn se0nak(self) -> &'a mut crate::W<REG> {
        self.variant(Prttstctl::Se0nak)
    }
    #[doc = "Test_Packet mode."]
    #[inline(always)]
    pub fn packet(self) -> &'a mut crate::W<REG> {
        self.variant(Prttstctl::Packet)
    }
    #[doc = "Test_Force_Enable."]
    #[inline(always)]
    pub fn force(self) -> &'a mut crate::W<REG> {
        self.variant(Prttstctl::Force)
    }
}
#[doc = "Port Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prtspd {
    #[doc = "1: Full speed."]
    Fs = 1,
    #[doc = "2: Low speed."]
    Ls = 2,
}
impl From<Prtspd> for u8 {
    #[inline(always)]
    fn from(variant: Prtspd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prtspd {
    type Ux = u8;
}
impl crate::IsEnum for Prtspd {}
#[doc = "Field `PRTSPD` reader - Port Speed"]
pub type PrtspdR = crate::FieldReader<Prtspd>;
impl PrtspdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prtspd> {
        match self.bits {
            1 => Some(Prtspd::Fs),
            2 => Some(Prtspd::Ls),
            _ => None,
        }
    }
    #[doc = "Full speed."]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        *self == Prtspd::Fs
    }
    #[doc = "Low speed."]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == Prtspd::Ls
    }
}
impl R {
    #[doc = "Bit 0 - Port Connect Status"]
    #[inline(always)]
    pub fn prtconnsts(&self) -> PrtconnstsR {
        PrtconnstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Connect Detected"]
    #[inline(always)]
    pub fn prtconndet(&self) -> PrtconndetR {
        PrtconndetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline(always)]
    pub fn prtena(&self) -> PrtenaR {
        PrtenaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline(always)]
    pub fn prtenchng(&self) -> PrtenchngR {
        PrtenchngR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Overcurrent Active"]
    #[inline(always)]
    pub fn prtovrcurract(&self) -> PrtovrcurractR {
        PrtovrcurractR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline(always)]
    pub fn prtovrcurrchng(&self) -> PrtovrcurrchngR {
        PrtovrcurrchngR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline(always)]
    pub fn prtres(&self) -> PrtresR {
        PrtresR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline(always)]
    pub fn prtsusp(&self) -> PrtsuspR {
        PrtsuspR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn prtrst(&self) -> PrtrstR {
        PrtrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Port Line Status"]
    #[inline(always)]
    pub fn prtlnsts(&self) -> PrtlnstsR {
        PrtlnstsR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    pub fn prtpwr(&self) -> PrtpwrR {
        PrtpwrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - Port Test Control"]
    #[inline(always)]
    pub fn prttstctl(&self) -> PrttstctlR {
        PrttstctlR::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:18 - Port Speed"]
    #[inline(always)]
    pub fn prtspd(&self) -> PrtspdR {
        PrtspdR::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Port Connect Detected"]
    #[inline(always)]
    pub fn prtconndet(&mut self) -> PrtconndetW<'_, HprtSpec> {
        PrtconndetW::new(self, 1)
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline(always)]
    pub fn prtena(&mut self) -> PrtenaW<'_, HprtSpec> {
        PrtenaW::new(self, 2)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline(always)]
    pub fn prtenchng(&mut self) -> PrtenchngW<'_, HprtSpec> {
        PrtenchngW::new(self, 3)
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline(always)]
    pub fn prtovrcurrchng(&mut self) -> PrtovrcurrchngW<'_, HprtSpec> {
        PrtovrcurrchngW::new(self, 5)
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline(always)]
    pub fn prtres(&mut self) -> PrtresW<'_, HprtSpec> {
        PrtresW::new(self, 6)
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline(always)]
    pub fn prtsusp(&mut self) -> PrtsuspW<'_, HprtSpec> {
        PrtsuspW::new(self, 7)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn prtrst(&mut self) -> PrtrstW<'_, HprtSpec> {
        PrtrstW::new(self, 8)
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    pub fn prtpwr(&mut self) -> PrtpwrW<'_, HprtSpec> {
        PrtpwrW::new(self, 12)
    }
    #[doc = "Bits 13:16 - Port Test Control"]
    #[inline(always)]
    pub fn prttstctl(&mut self) -> PrttstctlW<'_, HprtSpec> {
        PrttstctlW::new(self, 13)
    }
}
#[doc = "Host Port Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hprt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hprt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HprtSpec;
impl crate::RegisterSpec for HprtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hprt::R`](R) reader structure"]
impl crate::Readable for HprtSpec {}
#[doc = "`write(|w| ..)` method takes [`hprt::W`](W) writer structure"]
impl crate::Writable for HprtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HPRT to value 0"]
impl crate::Resettable for HprtSpec {}
