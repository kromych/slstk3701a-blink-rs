#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `CH0` reader - CH0 Interrupt Enable"]
pub type CH0_R = crate::BitReader;
#[doc = "Field `CH0` writer - CH0 Interrupt Enable"]
pub type CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - CH1 Interrupt Enable"]
pub type CH1_R = crate::BitReader;
#[doc = "Field `CH1` writer - CH1 Interrupt Enable"]
pub type CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - CH2 Interrupt Enable"]
pub type CH2_R = crate::BitReader;
#[doc = "Field `CH2` writer - CH2 Interrupt Enable"]
pub type CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` reader - CH3 Interrupt Enable"]
pub type CH3_R = crate::BitReader;
#[doc = "Field `CH3` writer - CH3 Interrupt Enable"]
pub type CH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` reader - CH4 Interrupt Enable"]
pub type CH4_R = crate::BitReader;
#[doc = "Field `CH4` writer - CH4 Interrupt Enable"]
pub type CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` reader - CH5 Interrupt Enable"]
pub type CH5_R = crate::BitReader;
#[doc = "Field `CH5` writer - CH5 Interrupt Enable"]
pub type CH5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` reader - CH6 Interrupt Enable"]
pub type CH6_R = crate::BitReader;
#[doc = "Field `CH6` writer - CH6 Interrupt Enable"]
pub type CH6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` reader - CH7 Interrupt Enable"]
pub type CH7_R = crate::BitReader;
#[doc = "Field `CH7` writer - CH7 Interrupt Enable"]
pub type CH7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8` reader - CH8 Interrupt Enable"]
pub type CH8_R = crate::BitReader;
#[doc = "Field `CH8` writer - CH8 Interrupt Enable"]
pub type CH8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9` reader - CH9 Interrupt Enable"]
pub type CH9_R = crate::BitReader;
#[doc = "Field `CH9` writer - CH9 Interrupt Enable"]
pub type CH9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10` reader - CH10 Interrupt Enable"]
pub type CH10_R = crate::BitReader;
#[doc = "Field `CH10` writer - CH10 Interrupt Enable"]
pub type CH10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11` reader - CH11 Interrupt Enable"]
pub type CH11_R = crate::BitReader;
#[doc = "Field `CH11` writer - CH11 Interrupt Enable"]
pub type CH11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH12` reader - CH12 Interrupt Enable"]
pub type CH12_R = crate::BitReader;
#[doc = "Field `CH12` writer - CH12 Interrupt Enable"]
pub type CH12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH13` reader - CH13 Interrupt Enable"]
pub type CH13_R = crate::BitReader;
#[doc = "Field `CH13` writer - CH13 Interrupt Enable"]
pub type CH13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH14` reader - CH14 Interrupt Enable"]
pub type CH14_R = crate::BitReader;
#[doc = "Field `CH14` writer - CH14 Interrupt Enable"]
pub type CH14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH15` reader - CH15 Interrupt Enable"]
pub type CH15_R = crate::BitReader;
#[doc = "Field `CH15` writer - CH15 Interrupt Enable"]
pub type CH15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANCOMPLETE` reader - SCANCOMPLETE Interrupt Enable"]
pub type SCANCOMPLETE_R = crate::BitReader;
#[doc = "Field `SCANCOMPLETE` writer - SCANCOMPLETE Interrupt Enable"]
pub type SCANCOMPLETE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEC` reader - DEC Interrupt Enable"]
pub type DEC_R = crate::BitReader;
#[doc = "Field `DEC` writer - DEC Interrupt Enable"]
pub type DEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DECERR` reader - DECERR Interrupt Enable"]
pub type DECERR_R = crate::BitReader;
#[doc = "Field `DECERR` writer - DECERR Interrupt Enable"]
pub type DECERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFDATAV` reader - BUFDATAV Interrupt Enable"]
pub type BUFDATAV_R = crate::BitReader;
#[doc = "Field `BUFDATAV` writer - BUFDATAV Interrupt Enable"]
pub type BUFDATAV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFLEVEL` reader - BUFLEVEL Interrupt Enable"]
pub type BUFLEVEL_R = crate::BitReader;
#[doc = "Field `BUFLEVEL` writer - BUFLEVEL Interrupt Enable"]
pub type BUFLEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFOF` reader - BUFOF Interrupt Enable"]
pub type BUFOF_R = crate::BitReader;
#[doc = "Field `BUFOF` writer - BUFOF Interrupt Enable"]
pub type BUFOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTOF` reader - CNTOF Interrupt Enable"]
pub type CNTOF_R = crate::BitReader;
#[doc = "Field `CNTOF` writer - CNTOF Interrupt Enable"]
pub type CNTOF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CH0 Interrupt Enable"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1 Interrupt Enable"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2 Interrupt Enable"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3 Interrupt Enable"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH4 Interrupt Enable"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH5 Interrupt Enable"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH6 Interrupt Enable"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CH7 Interrupt Enable"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CH8 Interrupt Enable"]
    #[inline(always)]
    pub fn ch8(&self) -> CH8_R {
        CH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CH9 Interrupt Enable"]
    #[inline(always)]
    pub fn ch9(&self) -> CH9_R {
        CH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CH10 Interrupt Enable"]
    #[inline(always)]
    pub fn ch10(&self) -> CH10_R {
        CH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CH11 Interrupt Enable"]
    #[inline(always)]
    pub fn ch11(&self) -> CH11_R {
        CH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CH12 Interrupt Enable"]
    #[inline(always)]
    pub fn ch12(&self) -> CH12_R {
        CH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CH13 Interrupt Enable"]
    #[inline(always)]
    pub fn ch13(&self) -> CH13_R {
        CH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CH14 Interrupt Enable"]
    #[inline(always)]
    pub fn ch14(&self) -> CH14_R {
        CH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CH15 Interrupt Enable"]
    #[inline(always)]
    pub fn ch15(&self) -> CH15_R {
        CH15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SCANCOMPLETE Interrupt Enable"]
    #[inline(always)]
    pub fn scancomplete(&self) -> SCANCOMPLETE_R {
        SCANCOMPLETE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DEC Interrupt Enable"]
    #[inline(always)]
    pub fn dec(&self) -> DEC_R {
        DEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DECERR Interrupt Enable"]
    #[inline(always)]
    pub fn decerr(&self) -> DECERR_R {
        DECERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - BUFDATAV Interrupt Enable"]
    #[inline(always)]
    pub fn bufdatav(&self) -> BUFDATAV_R {
        BUFDATAV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - BUFLEVEL Interrupt Enable"]
    #[inline(always)]
    pub fn buflevel(&self) -> BUFLEVEL_R {
        BUFLEVEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BUFOF Interrupt Enable"]
    #[inline(always)]
    pub fn bufof(&self) -> BUFOF_R {
        BUFOF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CNTOF Interrupt Enable"]
    #[inline(always)]
    pub fn cntof(&self) -> CNTOF_R {
        CNTOF_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<IEN_SPEC> {
        CH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - CH1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<IEN_SPEC> {
        CH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - CH2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<IEN_SPEC> {
        CH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - CH3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<IEN_SPEC> {
        CH3_W::new(self, 3)
    }
    #[doc = "Bit 4 - CH4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<IEN_SPEC> {
        CH4_W::new(self, 4)
    }
    #[doc = "Bit 5 - CH5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<IEN_SPEC> {
        CH5_W::new(self, 5)
    }
    #[doc = "Bit 6 - CH6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<IEN_SPEC> {
        CH6_W::new(self, 6)
    }
    #[doc = "Bit 7 - CH7 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<IEN_SPEC> {
        CH7_W::new(self, 7)
    }
    #[doc = "Bit 8 - CH8 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch8(&mut self) -> CH8_W<IEN_SPEC> {
        CH8_W::new(self, 8)
    }
    #[doc = "Bit 9 - CH9 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch9(&mut self) -> CH9_W<IEN_SPEC> {
        CH9_W::new(self, 9)
    }
    #[doc = "Bit 10 - CH10 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch10(&mut self) -> CH10_W<IEN_SPEC> {
        CH10_W::new(self, 10)
    }
    #[doc = "Bit 11 - CH11 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch11(&mut self) -> CH11_W<IEN_SPEC> {
        CH11_W::new(self, 11)
    }
    #[doc = "Bit 12 - CH12 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch12(&mut self) -> CH12_W<IEN_SPEC> {
        CH12_W::new(self, 12)
    }
    #[doc = "Bit 13 - CH13 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch13(&mut self) -> CH13_W<IEN_SPEC> {
        CH13_W::new(self, 13)
    }
    #[doc = "Bit 14 - CH14 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch14(&mut self) -> CH14_W<IEN_SPEC> {
        CH14_W::new(self, 14)
    }
    #[doc = "Bit 15 - CH15 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch15(&mut self) -> CH15_W<IEN_SPEC> {
        CH15_W::new(self, 15)
    }
    #[doc = "Bit 16 - SCANCOMPLETE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scancomplete(&mut self) -> SCANCOMPLETE_W<IEN_SPEC> {
        SCANCOMPLETE_W::new(self, 16)
    }
    #[doc = "Bit 17 - DEC Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dec(&mut self) -> DEC_W<IEN_SPEC> {
        DEC_W::new(self, 17)
    }
    #[doc = "Bit 18 - DECERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn decerr(&mut self) -> DECERR_W<IEN_SPEC> {
        DECERR_W::new(self, 18)
    }
    #[doc = "Bit 19 - BUFDATAV Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bufdatav(&mut self) -> BUFDATAV_W<IEN_SPEC> {
        BUFDATAV_W::new(self, 19)
    }
    #[doc = "Bit 20 - BUFLEVEL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn buflevel(&mut self) -> BUFLEVEL_W<IEN_SPEC> {
        BUFLEVEL_W::new(self, 20)
    }
    #[doc = "Bit 21 - BUFOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bufof(&mut self) -> BUFOF_W<IEN_SPEC> {
        BUFOF_W::new(self, 21)
    }
    #[doc = "Bit 22 - CNTOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cntof(&mut self) -> CNTOF_W<IEN_SPEC> {
        CNTOF_W::new(self, 22)
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
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
