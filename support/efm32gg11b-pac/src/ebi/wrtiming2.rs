#[doc = "Register `WRTIMING2` reader"]
pub type R = crate::R<WRTIMING2_SPEC>;
#[doc = "Register `WRTIMING2` writer"]
pub type W = crate::W<WRTIMING2_SPEC>;
#[doc = "Field `WRSETUP` reader - Write Setup Time"]
pub type WRSETUP_R = crate::FieldReader;
#[doc = "Field `WRSETUP` writer - Write Setup Time"]
pub type WRSETUP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WRSTRB` reader - Write Strobe Time"]
pub type WRSTRB_R = crate::FieldReader;
#[doc = "Field `WRSTRB` writer - Write Strobe Time"]
pub type WRSTRB_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRHOLD` reader - Write Hold Time"]
pub type WRHOLD_R = crate::FieldReader;
#[doc = "Field `WRHOLD` writer - Write Hold Time"]
pub type WRHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HALFWE` reader - Half Cycle WEn Strobe Duration Enable"]
pub type HALFWE_R = crate::BitReader;
#[doc = "Field `HALFWE` writer - Half Cycle WEn Strobe Duration Enable"]
pub type HALFWE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WBUFDIS` reader - Write Buffer Disable"]
pub type WBUFDIS_R = crate::BitReader;
#[doc = "Field `WBUFDIS` writer - Write Buffer Disable"]
pub type WBUFDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Write Setup Time"]
    #[inline(always)]
    pub fn wrsetup(&self) -> WRSETUP_R {
        WRSETUP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:14 - Write Strobe Time"]
    #[inline(always)]
    pub fn wrstrb(&self) -> WRSTRB_R {
        WRSTRB_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:18 - Write Hold Time"]
    #[inline(always)]
    pub fn wrhold(&self) -> WRHOLD_R {
        WRHOLD_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 28 - Half Cycle WEn Strobe Duration Enable"]
    #[inline(always)]
    pub fn halfwe(&self) -> HALFWE_R {
        HALFWE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Write Buffer Disable"]
    #[inline(always)]
    pub fn wbufdis(&self) -> WBUFDIS_R {
        WBUFDIS_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Write Setup Time"]
    #[inline(always)]
    #[must_use]
    pub fn wrsetup(&mut self) -> WRSETUP_W<WRTIMING2_SPEC> {
        WRSETUP_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Write Strobe Time"]
    #[inline(always)]
    #[must_use]
    pub fn wrstrb(&mut self) -> WRSTRB_W<WRTIMING2_SPEC> {
        WRSTRB_W::new(self, 8)
    }
    #[doc = "Bits 16:18 - Write Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn wrhold(&mut self) -> WRHOLD_W<WRTIMING2_SPEC> {
        WRHOLD_W::new(self, 16)
    }
    #[doc = "Bit 28 - Half Cycle WEn Strobe Duration Enable"]
    #[inline(always)]
    #[must_use]
    pub fn halfwe(&mut self) -> HALFWE_W<WRTIMING2_SPEC> {
        HALFWE_W::new(self, 28)
    }
    #[doc = "Bit 29 - Write Buffer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wbufdis(&mut self) -> WBUFDIS_W<WRTIMING2_SPEC> {
        WBUFDIS_W::new(self, 29)
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
#[doc = "Write Timing Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrtiming2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrtiming2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRTIMING2_SPEC;
impl crate::RegisterSpec for WRTIMING2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrtiming2::R`](R) reader structure"]
impl crate::Readable for WRTIMING2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wrtiming2::W`](W) writer structure"]
impl crate::Writable for WRTIMING2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRTIMING2 to value 0x0007_7f07"]
impl crate::Resettable for WRTIMING2_SPEC {
    const RESET_VALUE: u32 = 0x0007_7f07;
}
