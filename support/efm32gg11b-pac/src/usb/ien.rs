#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `VBUSDETH` reader - VBUSDETH Interrupt Enable"]
pub type VBUSDETH_R = crate::BitReader;
#[doc = "Field `VBUSDETH` writer - VBUSDETH Interrupt Enable"]
pub type VBUSDETH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSDETL` reader - VBUSDETL Interrupt Enable"]
pub type VBUSDETL_R = crate::BitReader;
#[doc = "Field `VBUSDETL` writer - VBUSDETL Interrupt Enable"]
pub type VBUSDETL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` reader - ERR Interrupt Enable"]
pub type ERR_R = crate::BitReader;
#[doc = "Field `ERR` writer - ERR Interrupt Enable"]
pub type ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCD` reader - DCD Interrupt Enable"]
pub type DCD_R = crate::BitReader;
#[doc = "Field `DCD` writer - DCD Interrupt Enable"]
pub type DCD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD` reader - PD Interrupt Enable"]
pub type PD_R = crate::BitReader;
#[doc = "Field `PD` writer - PD Interrupt Enable"]
pub type PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD` reader - SD Interrupt Enable"]
pub type SD_R = crate::BitReader;
#[doc = "Field `SD` writer - SD Interrupt Enable"]
pub type SD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VBUSDETH Interrupt Enable"]
    #[inline(always)]
    pub fn vbusdeth(&self) -> VBUSDETH_R {
        VBUSDETH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBUSDETL Interrupt Enable"]
    #[inline(always)]
    pub fn vbusdetl(&self) -> VBUSDETL_R {
        VBUSDETL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - ERR Interrupt Enable"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DCD Interrupt Enable"]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PD Interrupt Enable"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SD Interrupt Enable"]
    #[inline(always)]
    pub fn sd(&self) -> SD_R {
        SD_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUSDETH Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbusdeth(&mut self) -> VBUSDETH_W<IEN_SPEC> {
        VBUSDETH_W::new(self, 0)
    }
    #[doc = "Bit 1 - VBUSDETL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbusdetl(&mut self) -> VBUSDETL_W<IEN_SPEC> {
        VBUSDETL_W::new(self, 1)
    }
    #[doc = "Bit 8 - ERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<IEN_SPEC> {
        ERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - DCD Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcd(&mut self) -> DCD_W<IEN_SPEC> {
        DCD_W::new(self, 9)
    }
    #[doc = "Bit 10 - PD Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<IEN_SPEC> {
        PD_W::new(self, 10)
    }
    #[doc = "Bit 11 - SD Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sd(&mut self) -> SD_W<IEN_SPEC> {
        SD_W::new(self, 11)
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
