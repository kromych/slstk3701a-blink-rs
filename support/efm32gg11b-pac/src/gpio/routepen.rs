#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<ROUTEPEN_SPEC>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<ROUTEPEN_SPEC>;
#[doc = "Field `SWCLKTCKPEN` reader - Serial Wire Clock and JTAG Test Clock Pin Enable"]
pub type SWCLKTCKPEN_R = crate::BitReader;
#[doc = "Field `SWCLKTCKPEN` writer - Serial Wire Clock and JTAG Test Clock Pin Enable"]
pub type SWCLKTCKPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWDIOTMSPEN` reader - Serial Wire Data and JTAG Test Mode Select Pin Enable"]
pub type SWDIOTMSPEN_R = crate::BitReader;
#[doc = "Field `SWDIOTMSPEN` writer - Serial Wire Data and JTAG Test Mode Select Pin Enable"]
pub type SWDIOTMSPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TDOPEN` reader - JTAG Test Debug Output Pin Enable"]
pub type TDOPEN_R = crate::BitReader;
#[doc = "Field `TDOPEN` writer - JTAG Test Debug Output Pin Enable"]
pub type TDOPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TDIPEN` reader - JTAG Test Debug Input Pin Enable"]
pub type TDIPEN_R = crate::BitReader;
#[doc = "Field `TDIPEN` writer - JTAG Test Debug Input Pin Enable"]
pub type TDIPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWVPEN` reader - Serial Wire Viewer Output Pin Enable"]
pub type SWVPEN_R = crate::BitReader;
#[doc = "Field `SWVPEN` writer - Serial Wire Viewer Output Pin Enable"]
pub type SWVPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETMTCLKPEN` reader - ETM Trace Clock Pin Enable"]
pub type ETMTCLKPEN_R = crate::BitReader;
#[doc = "Field `ETMTCLKPEN` writer - ETM Trace Clock Pin Enable"]
pub type ETMTCLKPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETMTD0PEN` reader - ETM Trace Data Pin Enable"]
pub type ETMTD0PEN_R = crate::BitReader;
#[doc = "Field `ETMTD0PEN` writer - ETM Trace Data Pin Enable"]
pub type ETMTD0PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETMTD1PEN` reader - ETM Trace Data Pin Enable"]
pub type ETMTD1PEN_R = crate::BitReader;
#[doc = "Field `ETMTD1PEN` writer - ETM Trace Data Pin Enable"]
pub type ETMTD1PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETMTD2PEN` reader - ETM Trace Data Pin Enable"]
pub type ETMTD2PEN_R = crate::BitReader;
#[doc = "Field `ETMTD2PEN` writer - ETM Trace Data Pin Enable"]
pub type ETMTD2PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETMTD3PEN` reader - ETM Trace Data Pin Enable"]
pub type ETMTD3PEN_R = crate::BitReader;
#[doc = "Field `ETMTD3PEN` writer - ETM Trace Data Pin Enable"]
pub type ETMTD3PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Serial Wire Clock and JTAG Test Clock Pin Enable"]
    #[inline(always)]
    pub fn swclktckpen(&self) -> SWCLKTCKPEN_R {
        SWCLKTCKPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial Wire Data and JTAG Test Mode Select Pin Enable"]
    #[inline(always)]
    pub fn swdiotmspen(&self) -> SWDIOTMSPEN_R {
        SWDIOTMSPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - JTAG Test Debug Output Pin Enable"]
    #[inline(always)]
    pub fn tdopen(&self) -> TDOPEN_R {
        TDOPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - JTAG Test Debug Input Pin Enable"]
    #[inline(always)]
    pub fn tdipen(&self) -> TDIPEN_R {
        TDIPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Serial Wire Viewer Output Pin Enable"]
    #[inline(always)]
    pub fn swvpen(&self) -> SWVPEN_R {
        SWVPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - ETM Trace Clock Pin Enable"]
    #[inline(always)]
    pub fn etmtclkpen(&self) -> ETMTCLKPEN_R {
        ETMTCLKPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn etmtd0pen(&self) -> ETMTD0PEN_R {
        ETMTD0PEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn etmtd1pen(&self) -> ETMTD1PEN_R {
        ETMTD1PEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn etmtd2pen(&self) -> ETMTD2PEN_R {
        ETMTD2PEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn etmtd3pen(&self) -> ETMTD3PEN_R {
        ETMTD3PEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Wire Clock and JTAG Test Clock Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swclktckpen(&mut self) -> SWCLKTCKPEN_W<ROUTEPEN_SPEC, 0> {
        SWCLKTCKPEN_W::new(self)
    }
    #[doc = "Bit 1 - Serial Wire Data and JTAG Test Mode Select Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swdiotmspen(&mut self) -> SWDIOTMSPEN_W<ROUTEPEN_SPEC, 1> {
        SWDIOTMSPEN_W::new(self)
    }
    #[doc = "Bit 2 - JTAG Test Debug Output Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdopen(&mut self) -> TDOPEN_W<ROUTEPEN_SPEC, 2> {
        TDOPEN_W::new(self)
    }
    #[doc = "Bit 3 - JTAG Test Debug Input Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdipen(&mut self) -> TDIPEN_W<ROUTEPEN_SPEC, 3> {
        TDIPEN_W::new(self)
    }
    #[doc = "Bit 4 - Serial Wire Viewer Output Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swvpen(&mut self) -> SWVPEN_W<ROUTEPEN_SPEC, 4> {
        SWVPEN_W::new(self)
    }
    #[doc = "Bit 16 - ETM Trace Clock Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn etmtclkpen(&mut self) -> ETMTCLKPEN_W<ROUTEPEN_SPEC, 16> {
        ETMTCLKPEN_W::new(self)
    }
    #[doc = "Bit 17 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn etmtd0pen(&mut self) -> ETMTD0PEN_W<ROUTEPEN_SPEC, 17> {
        ETMTD0PEN_W::new(self)
    }
    #[doc = "Bit 18 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn etmtd1pen(&mut self) -> ETMTD1PEN_W<ROUTEPEN_SPEC, 18> {
        ETMTD1PEN_W::new(self)
    }
    #[doc = "Bit 19 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn etmtd2pen(&mut self) -> ETMTD2PEN_W<ROUTEPEN_SPEC, 19> {
        ETMTD2PEN_W::new(self)
    }
    #[doc = "Bit 20 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn etmtd3pen(&mut self) -> ETMTD3PEN_W<ROUTEPEN_SPEC, 20> {
        ETMTD3PEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I/O Routing Pin Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routepen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routepen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTEPEN_SPEC;
impl crate::RegisterSpec for ROUTEPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routepen::R`](R) reader structure"]
impl crate::Readable for ROUTEPEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`routepen::W`](W) writer structure"]
impl crate::Writable for ROUTEPEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0x0f"]
impl crate::Resettable for ROUTEPEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
