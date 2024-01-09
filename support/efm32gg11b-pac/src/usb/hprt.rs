#[doc = "Register `HPRT` reader"]
pub type R = crate::R<HPRT_SPEC>;
#[doc = "Register `HPRT` writer"]
pub type W = crate::W<HPRT_SPEC>;
#[doc = "Field `PRTCONNSTS` reader - Port Connect Status"]
pub type PRTCONNSTS_R = crate::BitReader;
#[doc = "Field `PRTCONNDET` reader - Port Connect Detected"]
pub type PRTCONNDET_R = crate::BitReader;
#[doc = "Field `PRTCONNDET` writer - Port Connect Detected"]
pub type PRTCONNDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTENA` reader - Port Enable"]
pub type PRTENA_R = crate::BitReader;
#[doc = "Field `PRTENA` writer - Port Enable"]
pub type PRTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTENCHNG` reader - Port Enable/Disable Change"]
pub type PRTENCHNG_R = crate::BitReader;
#[doc = "Field `PRTENCHNG` writer - Port Enable/Disable Change"]
pub type PRTENCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTOVRCURRACT` reader - Port Overcurrent Active"]
pub type PRTOVRCURRACT_R = crate::BitReader;
#[doc = "Field `PRTOVRCURRCHNG` reader - Port Overcurrent Change"]
pub type PRTOVRCURRCHNG_R = crate::BitReader;
#[doc = "Field `PRTOVRCURRCHNG` writer - Port Overcurrent Change"]
pub type PRTOVRCURRCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTRES` reader - Port Resume"]
pub type PRTRES_R = crate::BitReader;
#[doc = "Field `PRTRES` writer - Port Resume"]
pub type PRTRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTSUSP` reader - Port Suspend"]
pub type PRTSUSP_R = crate::BitReader;
#[doc = "Field `PRTSUSP` writer - Port Suspend"]
pub type PRTSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTRST` reader - Port Reset"]
pub type PRTRST_R = crate::BitReader;
#[doc = "Field `PRTRST` writer - Port Reset"]
pub type PRTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTLNSTS` reader - Port Line Status"]
pub type PRTLNSTS_R = crate::FieldReader;
#[doc = "Field `PRTPWR` reader - Port Power"]
pub type PRTPWR_R = crate::BitReader;
#[doc = "Field `PRTPWR` writer - Port Power"]
pub type PRTPWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTTSTCTL` reader - Port Test Control"]
pub type PRTTSTCTL_R = crate::FieldReader<PRTTSTCTL_A>;
#[doc = "Port Test Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRTTSTCTL_A {
    #[doc = "0: Test mode disabled."]
    DISABLE = 0,
    #[doc = "1: Test_J mode."]
    J = 1,
    #[doc = "2: Test_K mode."]
    K = 2,
    #[doc = "3: Test_SE0_NAK mode."]
    SE0NAK = 3,
    #[doc = "4: Test_Packet mode."]
    PACKET = 4,
    #[doc = "5: Test_Force_Enable."]
    FORCE = 5,
}
impl From<PRTTSTCTL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRTTSTCTL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRTTSTCTL_A {
    type Ux = u8;
}
impl PRTTSTCTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRTTSTCTL_A> {
        match self.bits {
            0 => Some(PRTTSTCTL_A::DISABLE),
            1 => Some(PRTTSTCTL_A::J),
            2 => Some(PRTTSTCTL_A::K),
            3 => Some(PRTTSTCTL_A::SE0NAK),
            4 => Some(PRTTSTCTL_A::PACKET),
            5 => Some(PRTTSTCTL_A::FORCE),
            _ => None,
        }
    }
    #[doc = "Test mode disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PRTTSTCTL_A::DISABLE
    }
    #[doc = "Test_J mode."]
    #[inline(always)]
    pub fn is_j(&self) -> bool {
        *self == PRTTSTCTL_A::J
    }
    #[doc = "Test_K mode."]
    #[inline(always)]
    pub fn is_k(&self) -> bool {
        *self == PRTTSTCTL_A::K
    }
    #[doc = "Test_SE0_NAK mode."]
    #[inline(always)]
    pub fn is_se0nak(&self) -> bool {
        *self == PRTTSTCTL_A::SE0NAK
    }
    #[doc = "Test_Packet mode."]
    #[inline(always)]
    pub fn is_packet(&self) -> bool {
        *self == PRTTSTCTL_A::PACKET
    }
    #[doc = "Test_Force_Enable."]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == PRTTSTCTL_A::FORCE
    }
}
#[doc = "Field `PRTTSTCTL` writer - Port Test Control"]
pub type PRTTSTCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PRTTSTCTL_A>;
impl<'a, REG> PRTTSTCTL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Test mode disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PRTTSTCTL_A::DISABLE)
    }
    #[doc = "Test_J mode."]
    #[inline(always)]
    pub fn j(self) -> &'a mut crate::W<REG> {
        self.variant(PRTTSTCTL_A::J)
    }
    #[doc = "Test_K mode."]
    #[inline(always)]
    pub fn k(self) -> &'a mut crate::W<REG> {
        self.variant(PRTTSTCTL_A::K)
    }
    #[doc = "Test_SE0_NAK mode."]
    #[inline(always)]
    pub fn se0nak(self) -> &'a mut crate::W<REG> {
        self.variant(PRTTSTCTL_A::SE0NAK)
    }
    #[doc = "Test_Packet mode."]
    #[inline(always)]
    pub fn packet(self) -> &'a mut crate::W<REG> {
        self.variant(PRTTSTCTL_A::PACKET)
    }
    #[doc = "Test_Force_Enable."]
    #[inline(always)]
    pub fn force(self) -> &'a mut crate::W<REG> {
        self.variant(PRTTSTCTL_A::FORCE)
    }
}
#[doc = "Field `PRTSPD` reader - Port Speed"]
pub type PRTSPD_R = crate::FieldReader<PRTSPD_A>;
#[doc = "Port Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRTSPD_A {
    #[doc = "1: Full speed."]
    FS = 1,
    #[doc = "2: Low speed."]
    LS = 2,
}
impl From<PRTSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: PRTSPD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRTSPD_A {
    type Ux = u8;
}
impl PRTSPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRTSPD_A> {
        match self.bits {
            1 => Some(PRTSPD_A::FS),
            2 => Some(PRTSPD_A::LS),
            _ => None,
        }
    }
    #[doc = "Full speed."]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        *self == PRTSPD_A::FS
    }
    #[doc = "Low speed."]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == PRTSPD_A::LS
    }
}
impl R {
    #[doc = "Bit 0 - Port Connect Status"]
    #[inline(always)]
    pub fn prtconnsts(&self) -> PRTCONNSTS_R {
        PRTCONNSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Connect Detected"]
    #[inline(always)]
    pub fn prtconndet(&self) -> PRTCONNDET_R {
        PRTCONNDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline(always)]
    pub fn prtena(&self) -> PRTENA_R {
        PRTENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline(always)]
    pub fn prtenchng(&self) -> PRTENCHNG_R {
        PRTENCHNG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Overcurrent Active"]
    #[inline(always)]
    pub fn prtovrcurract(&self) -> PRTOVRCURRACT_R {
        PRTOVRCURRACT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline(always)]
    pub fn prtovrcurrchng(&self) -> PRTOVRCURRCHNG_R {
        PRTOVRCURRCHNG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline(always)]
    pub fn prtres(&self) -> PRTRES_R {
        PRTRES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline(always)]
    pub fn prtsusp(&self) -> PRTSUSP_R {
        PRTSUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn prtrst(&self) -> PRTRST_R {
        PRTRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Port Line Status"]
    #[inline(always)]
    pub fn prtlnsts(&self) -> PRTLNSTS_R {
        PRTLNSTS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    pub fn prtpwr(&self) -> PRTPWR_R {
        PRTPWR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - Port Test Control"]
    #[inline(always)]
    pub fn prttstctl(&self) -> PRTTSTCTL_R {
        PRTTSTCTL_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:18 - Port Speed"]
    #[inline(always)]
    pub fn prtspd(&self) -> PRTSPD_R {
        PRTSPD_R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Port Connect Detected"]
    #[inline(always)]
    #[must_use]
    pub fn prtconndet(&mut self) -> PRTCONNDET_W<HPRT_SPEC> {
        PRTCONNDET_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prtena(&mut self) -> PRTENA_W<HPRT_SPEC> {
        PRTENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline(always)]
    #[must_use]
    pub fn prtenchng(&mut self) -> PRTENCHNG_W<HPRT_SPEC> {
        PRTENCHNG_W::new(self, 3)
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline(always)]
    #[must_use]
    pub fn prtovrcurrchng(&mut self) -> PRTOVRCURRCHNG_W<HPRT_SPEC> {
        PRTOVRCURRCHNG_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline(always)]
    #[must_use]
    pub fn prtres(&mut self) -> PRTRES_W<HPRT_SPEC> {
        PRTRES_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn prtsusp(&mut self) -> PRTSUSP_W<HPRT_SPEC> {
        PRTSUSP_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    #[must_use]
    pub fn prtrst(&mut self) -> PRTRST_W<HPRT_SPEC> {
        PRTRST_W::new(self, 8)
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    #[must_use]
    pub fn prtpwr(&mut self) -> PRTPWR_W<HPRT_SPEC> {
        PRTPWR_W::new(self, 12)
    }
    #[doc = "Bits 13:16 - Port Test Control"]
    #[inline(always)]
    #[must_use]
    pub fn prttstctl(&mut self) -> PRTTSTCTL_W<HPRT_SPEC> {
        PRTTSTCTL_W::new(self, 13)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Host Port Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hprt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hprt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPRT_SPEC;
impl crate::RegisterSpec for HPRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hprt::R`](R) reader structure"]
impl crate::Readable for HPRT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hprt::W`](W) writer structure"]
impl crate::Writable for HPRT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPRT to value 0"]
impl crate::Resettable for HPRT_SPEC {
    const RESET_VALUE: u32 = 0;
}
