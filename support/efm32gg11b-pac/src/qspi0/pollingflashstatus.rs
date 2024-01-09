#[doc = "Register `POLLINGFLASHSTATUS` reader"]
pub type R = crate::R<POLLINGFLASHSTATUS_SPEC>;
#[doc = "Register `POLLINGFLASHSTATUS` writer"]
pub type W = crate::W<POLLINGFLASHSTATUS_SPEC>;
#[doc = "Field `DEVICESTATUS` reader - Device Status"]
pub type DEVICESTATUS_R = crate::FieldReader;
#[doc = "Field `DEVICESTATUSVALID` reader - Device Status Valid"]
pub type DEVICESTATUSVALID_R = crate::BitReader;
#[doc = "Field `DEVICESTATUSNBDUMMY` reader - Auto-polling Dummy Cycles"]
pub type DEVICESTATUSNBDUMMY_R = crate::FieldReader;
#[doc = "Field `DEVICESTATUSNBDUMMY` writer - Auto-polling Dummy Cycles"]
pub type DEVICESTATUSNBDUMMY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - Device Status"]
    #[inline(always)]
    pub fn devicestatus(&self) -> DEVICESTATUS_R {
        DEVICESTATUS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Device Status Valid"]
    #[inline(always)]
    pub fn devicestatusvalid(&self) -> DEVICESTATUSVALID_R {
        DEVICESTATUSVALID_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Auto-polling Dummy Cycles"]
    #[inline(always)]
    pub fn devicestatusnbdummy(&self) -> DEVICESTATUSNBDUMMY_R {
        DEVICESTATUSNBDUMMY_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - Auto-polling Dummy Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn devicestatusnbdummy(&mut self) -> DEVICESTATUSNBDUMMY_W<POLLINGFLASHSTATUS_SPEC> {
        DEVICESTATUSNBDUMMY_W::new(self, 16)
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
#[doc = "Polling Flash Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pollingflashstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pollingflashstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POLLINGFLASHSTATUS_SPEC;
impl crate::RegisterSpec for POLLINGFLASHSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pollingflashstatus::R`](R) reader structure"]
impl crate::Readable for POLLINGFLASHSTATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pollingflashstatus::W`](W) writer structure"]
impl crate::Writable for POLLINGFLASHSTATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POLLINGFLASHSTATUS to value 0"]
impl crate::Resettable for POLLINGFLASHSTATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
