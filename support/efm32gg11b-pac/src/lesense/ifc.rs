#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFC_SPEC>;
#[doc = "Field `CH0` writer - Clear CH0 Interrupt Flag"]
pub type CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` writer - Clear CH1 Interrupt Flag"]
pub type CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` writer - Clear CH2 Interrupt Flag"]
pub type CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` writer - Clear CH3 Interrupt Flag"]
pub type CH3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` writer - Clear CH4 Interrupt Flag"]
pub type CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` writer - Clear CH5 Interrupt Flag"]
pub type CH5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` writer - Clear CH6 Interrupt Flag"]
pub type CH6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` writer - Clear CH7 Interrupt Flag"]
pub type CH7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8` writer - Clear CH8 Interrupt Flag"]
pub type CH8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9` writer - Clear CH9 Interrupt Flag"]
pub type CH9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10` writer - Clear CH10 Interrupt Flag"]
pub type CH10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11` writer - Clear CH11 Interrupt Flag"]
pub type CH11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH12` writer - Clear CH12 Interrupt Flag"]
pub type CH12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH13` writer - Clear CH13 Interrupt Flag"]
pub type CH13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH14` writer - Clear CH14 Interrupt Flag"]
pub type CH14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH15` writer - Clear CH15 Interrupt Flag"]
pub type CH15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANCOMPLETE` writer - Clear SCANCOMPLETE Interrupt Flag"]
pub type SCANCOMPLETE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEC` writer - Clear DEC Interrupt Flag"]
pub type DEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DECERR` writer - Clear DECERR Interrupt Flag"]
pub type DECERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFDATAV` writer - Clear BUFDATAV Interrupt Flag"]
pub type BUFDATAV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFLEVEL` writer - Clear BUFLEVEL Interrupt Flag"]
pub type BUFLEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFOF` writer - Clear BUFOF Interrupt Flag"]
pub type BUFOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTOF` writer - Clear CNTOF Interrupt Flag"]
pub type CNTOF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear CH0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<IFC_SPEC> {
        CH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear CH1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<IFC_SPEC> {
        CH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear CH2 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<IFC_SPEC> {
        CH2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear CH3 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<IFC_SPEC> {
        CH3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear CH4 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<IFC_SPEC> {
        CH4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear CH5 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<IFC_SPEC> {
        CH5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear CH6 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<IFC_SPEC> {
        CH6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear CH7 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<IFC_SPEC> {
        CH7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear CH8 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch8(&mut self) -> CH8_W<IFC_SPEC> {
        CH8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear CH9 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch9(&mut self) -> CH9_W<IFC_SPEC> {
        CH9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear CH10 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch10(&mut self) -> CH10_W<IFC_SPEC> {
        CH10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear CH11 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch11(&mut self) -> CH11_W<IFC_SPEC> {
        CH11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear CH12 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch12(&mut self) -> CH12_W<IFC_SPEC> {
        CH12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear CH13 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch13(&mut self) -> CH13_W<IFC_SPEC> {
        CH13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear CH14 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch14(&mut self) -> CH14_W<IFC_SPEC> {
        CH14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear CH15 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch15(&mut self) -> CH15_W<IFC_SPEC> {
        CH15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Clear SCANCOMPLETE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn scancomplete(&mut self) -> SCANCOMPLETE_W<IFC_SPEC> {
        SCANCOMPLETE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear DEC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dec(&mut self) -> DEC_W<IFC_SPEC> {
        DEC_W::new(self, 17)
    }
    #[doc = "Bit 18 - Clear DECERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn decerr(&mut self) -> DECERR_W<IFC_SPEC> {
        DECERR_W::new(self, 18)
    }
    #[doc = "Bit 19 - Clear BUFDATAV Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bufdatav(&mut self) -> BUFDATAV_W<IFC_SPEC> {
        BUFDATAV_W::new(self, 19)
    }
    #[doc = "Bit 20 - Clear BUFLEVEL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn buflevel(&mut self) -> BUFLEVEL_W<IFC_SPEC> {
        BUFLEVEL_W::new(self, 20)
    }
    #[doc = "Bit 21 - Clear BUFOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bufof(&mut self) -> BUFOF_W<IFC_SPEC> {
        BUFOF_W::new(self, 21)
    }
    #[doc = "Bit 22 - Clear CNTOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cntof(&mut self) -> CNTOF_W<IFC_SPEC> {
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
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    const RESET_VALUE: u32 = 0;
}
