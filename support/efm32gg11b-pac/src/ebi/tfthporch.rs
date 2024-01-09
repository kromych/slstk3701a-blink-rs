#[doc = "Register `TFTHPORCH` reader"]
pub type R = crate::R<TFTHPORCH_SPEC>;
#[doc = "Register `TFTHPORCH` writer"]
pub type W = crate::W<TFTHPORCH_SPEC>;
#[doc = "Field `HSYNC` reader - Horizontal Synchronization Pulse Width"]
pub type HSYNC_R = crate::FieldReader;
#[doc = "Field `HSYNC` writer - Horizontal Synchronization Pulse Width"]
pub type HSYNC_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HFPORCH` reader - Horizontal Front Porch Size"]
pub type HFPORCH_R = crate::FieldReader;
#[doc = "Field `HFPORCH` writer - Horizontal Front Porch Size"]
pub type HFPORCH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HBPORCH` reader - Horizontal Back Porch Size"]
pub type HBPORCH_R = crate::FieldReader;
#[doc = "Field `HBPORCH` writer - Horizontal Back Porch Size"]
pub type HBPORCH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HSYNCSTART` reader - HSYNC Start Delay"]
pub type HSYNCSTART_R = crate::FieldReader;
#[doc = "Field `HSYNCSTART` writer - HSYNC Start Delay"]
pub type HSYNCSTART_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - Horizontal Synchronization Pulse Width"]
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Horizontal Front Porch Size"]
    #[inline(always)]
    pub fn hfporch(&self) -> HFPORCH_R {
        HFPORCH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 18:25 - Horizontal Back Porch Size"]
    #[inline(always)]
    pub fn hbporch(&self) -> HBPORCH_R {
        HBPORCH_R::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bits 28:29 - HSYNC Start Delay"]
    #[inline(always)]
    pub fn hsyncstart(&self) -> HSYNCSTART_R {
        HSYNCSTART_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Horizontal Synchronization Pulse Width"]
    #[inline(always)]
    #[must_use]
    pub fn hsync(&mut self) -> HSYNC_W<TFTHPORCH_SPEC> {
        HSYNC_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Horizontal Front Porch Size"]
    #[inline(always)]
    #[must_use]
    pub fn hfporch(&mut self) -> HFPORCH_W<TFTHPORCH_SPEC> {
        HFPORCH_W::new(self, 8)
    }
    #[doc = "Bits 18:25 - Horizontal Back Porch Size"]
    #[inline(always)]
    #[must_use]
    pub fn hbporch(&mut self) -> HBPORCH_W<TFTHPORCH_SPEC> {
        HBPORCH_W::new(self, 18)
    }
    #[doc = "Bits 28:29 - HSYNC Start Delay"]
    #[inline(always)]
    #[must_use]
    pub fn hsyncstart(&mut self) -> HSYNCSTART_W<TFTHPORCH_SPEC> {
        HSYNCSTART_W::new(self, 28)
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
#[doc = "TFT Horizontal Porch Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfthporch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfthporch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TFTHPORCH_SPEC;
impl crate::RegisterSpec for TFTHPORCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfthporch::R`](R) reader structure"]
impl crate::Readable for TFTHPORCH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tfthporch::W`](W) writer structure"]
impl crate::Writable for TFTHPORCH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TFTHPORCH to value 0"]
impl crate::Resettable for TFTHPORCH_SPEC {
    const RESET_VALUE: u32 = 0;
}
