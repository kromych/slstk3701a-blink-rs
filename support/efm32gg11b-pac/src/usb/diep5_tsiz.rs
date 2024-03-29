#[doc = "Register `DIEP5_TSIZ` reader"]
pub type R = crate::R<DIEP5_TSIZ_SPEC>;
#[doc = "Register `DIEP5_TSIZ` writer"]
pub type W = crate::W<DIEP5_TSIZ_SPEC>;
#[doc = "Field `XFERSIZE` reader - Transfer Size"]
pub type XFERSIZE_R = crate::FieldReader<u32>;
#[doc = "Field `XFERSIZE` writer - Transfer Size"]
pub type XFERSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - Packet Count"]
pub type PKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - Packet Count"]
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `MC` reader - Multi Count"]
pub type MC_R = crate::FieldReader;
#[doc = "Field `MC` writer - Multi Count"]
pub type MC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&self) -> XFERSIZE_R {
        XFERSIZE_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - Multi Count"]
    #[inline(always)]
    pub fn mc(&self) -> MC_R {
        MC_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize(&mut self) -> XFERSIZE_W<DIEP5_TSIZ_SPEC> {
        XFERSIZE_W::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<DIEP5_TSIZ_SPEC> {
        PKTCNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30 - Multi Count"]
    #[inline(always)]
    #[must_use]
    pub fn mc(&mut self) -> MC_W<DIEP5_TSIZ_SPEC> {
        MC_W::new(self, 29)
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
#[doc = "Device IN Endpoint x+1 Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diep5_tsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diep5_tsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEP5_TSIZ_SPEC;
impl crate::RegisterSpec for DIEP5_TSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diep5_tsiz::R`](R) reader structure"]
impl crate::Readable for DIEP5_TSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diep5_tsiz::W`](W) writer structure"]
impl crate::Writable for DIEP5_TSIZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEP5_TSIZ to value 0"]
impl crate::Resettable for DIEP5_TSIZ_SPEC {
    const RESET_VALUE: u32 = 0;
}
